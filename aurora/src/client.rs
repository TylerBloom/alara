use serde::Deserialize;
use tokio::{
    io::{AsyncBufReadExt, BufReader, Lines, Stdin},
    sync::mpsc::{self, error::TryRecvError, UnboundedReceiver},
};

use crate::{InitBody, Message, MessageBody, Node};

/// The main loop for problem solutions to use. This handles creating the client and node, looping
/// until the process is killed, and pulling, processing, and sending messages. Of course,
/// solutions can implement whatever loop they want.
pub async fn main_loop<N: Node>() {
    let (mut client, mut node): (_, N) = Client::new().await;
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 2 {
            println!(r#"{{"src":"n0","dest":"n0","body":{{"type":"read_ok","messages":[]}}}}"#);
            continue;
        }
        let msg = match client.recv.as_mut() {
            None => read_msg(&mut client.stdin).await,
            Some(recv) => {
                tokio::select! {
                    msg = read_msg(&mut client.stdin) => msg,
                    _msg = recv.recv() => {
                        panic!("No message should be sent over channel yet...");
                        //let msg = msg.expect("node hung up (likely crashed)");
                        //client.send_msg(msg);
                        //continue
                    }
                }
            }
        };
        if counter == 2 {
            panic!("Waiting for a second message...");
        }
        let Ok(Some(msg)) = node.handle_msg(msg) else { continue };
        client.send_msg(msg);
    }
}

/// The main client used to receive new messages and send processed responses.
/// Received messages can come from either stdin or from the sender half of the channel that the
/// node receives upon construction.
/// If the node does not use the sender, the receiver is never checked for messages.
///
/// NOTE: The node does *not* need to use the sender half of the channel. The channel is intended
/// to messsages through the client asynchronously.
#[derive(Debug)]
pub struct Client<N: Node> {
    stdin: Lines<BufReader<Stdin>>,
    recv: Option<UnboundedReceiver<Message<N::Body>>>,
}

const INIT_ERR_MSG: &str = "init message not given";
const STDIN_ERR_MSG: &str = "failed to read line from stdin";
const DE_ERR_MSG: &str = "failed to parse given message";
const SER_ERR_MSG: &str = "failed to serialize given message";

/// The client can receive `Init` and `InitOk` messages in addition to messages with the node's
/// expected data. To ensure that we don't ignore malformed messages, this type helps abstract over
/// those possiblities.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged, bound = "B: MessageBody")]
pub enum OrInit<B: MessageBody> {
    /// The main type that the node is expecting
    Main(Message<B>),
    /// An `Init` or `InitOk` message
    Init(Message<InitBody>),
}

impl<N: Node> Client<N> {
    /// Creates a new client, waits to receive an `Init` message, constructs the node, and then
    /// return both the client and node.
    pub async fn new() -> (Self, N) {
        let mut stdin = BufReader::new(tokio::io::stdin()).lines();
        let raw_init: String = stdin
            .next_line()
            .await
            .expect(INIT_ERR_MSG)
            .expect(STDIN_ERR_MSG);
        let init: Message<InitBody> =
            serde_json::from_str(&raw_init).expect("failed to parse init message");
        let Message { src, dest, body } = init;
        let InitBody::Init { msg_id, node_id, node_ids } = body else {
            panic!("first message in stdout was not an init message")
        };
        let (send, mut recv) = mpsc::unbounded_channel();
        let mut node = N::init(send, node_id, node_ids);
        let recv = recv
            .try_recv()
            .map_err(|err| (err == TryRecvError::Empty).then_some(recv))
            .expect_err("nodes should not send messages during construction");
        let mut digest = Self { stdin, recv };
        let resp = Message {
            src: dest,
            dest: src,
            body: InitBody::InitOk {
                msg_id: node.next_id(),
                in_reply_to: msg_id,
            },
        };
        //println!("Sending InitOk message...");
        digest.send_msg(resp);
        (digest, node)
    }

    /// Waits for the next message to arrive over stdin.
    ///
    /// NOTE: This method does *not* check the channel. That logic is handled by the `main_loop`.
    pub async fn next_msg(&mut self) -> Message<N::Body> {
        read_msg(&mut self.stdin).await
    }

    /// Sends a message over stdout.
    pub fn send_msg<B>(&mut self, msg: Message<B>)
    where
        B: MessageBody,
    {
        println!("{}", serde_json::to_string(&msg).expect(SER_ERR_MSG));
    }
}

/// Reads messages from stdin. Messages can either be a `InitOk` message or a message of the
/// specified type. `InitOk` messages are ignored and `Init` messages cause panics. Otherwise, the
/// message is returned
async fn read_msg<B: MessageBody>(stdin: &mut Lines<BufReader<Stdin>>) -> Message<B> {
    loop {
        match stdin.next_line().await {
            Err(err) => {
                panic!("error while reading line: {err}")
            }
            Ok(None) => {
                panic!("read nothing from stdin");
                //continue
            }
            Ok(Some(line)) => {
                let val: OrInit<B> =
                    serde_json::from_str(&line).expect(&format!("{DE_ERR_MSG}: {line}"));
                match val {
                    OrInit::Main(msg) => return msg,
                    OrInit::Init(Message {
                        body: InitBody::InitOk { .. },
                        ..
                    }) => continue,
                    OrInit::Init(Message {
                        body: InitBody::Init { .. },
                        ..
                    }) => panic!("mutliple init messages recieved"),
                }
            }
        }
    }
}
