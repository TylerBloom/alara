use std::io::{self, StdoutLock, Write};

use either::Either;
use serde::Deserialize;
use tokio::{
    io::{AsyncBufReadExt, BufReader, Lines, Stdin},
    sync::mpsc::{self, error::TryRecvError, UnboundedReceiver},
};

use crate::{InitBody, Message, MessageBody, Node};

pub async fn main_loop<N: Node>(mut client: Client<N>, mut node: N) {
    loop {
        let msg = client.next_msg().await;
        let Ok(Some(msg)) = node.handle_msg(msg) else { continue };
        client.send_msg(msg);
    }
}

pub struct Client<N: Node> {
    stdin: Lines<BufReader<Stdin>>,
    recv: Option<UnboundedReceiver<Message<N::Body>>>,
}

pub const INIT_ERR_MSG: &str = "init message not given";
pub const STDIN_ERR_MSG: &str = "failed to read line from stdin";
pub const STDOUT_ERR_MSG: &str = "failed to write line from stdin";
pub const DE_ERR_MSG: &str = "failed to parse given message";
pub const SER_ERR_MSG: &str = "failed to serialize given message";

#[derive(Deserialize)]
#[serde(transparent)]
struct OrInit<B: MessageBody>(
    #[serde(with = "either::serde_untagged")] pub Either<Message<B>, Message<InitBody>>,
);

impl<N: Node> Client<N> {
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
        let InitBody::Init { id, node_id, node_ids } = body else {
            panic!("first message in stdout was not an init message")
        };
        let (send, mut recv) = mpsc::unbounded_channel();
        let mut node = N::init(send, node_id, node_ids);
        let recv = match recv.try_recv() {
            Err(TryRecvError::Disconnected) => None,
            _ => panic!("No nodes types should be holding onto the sender yet"),
        };
        let mut digest = Self {
            stdin,
            recv,
        };
        let resp = Message {
            src: dest,
            dest: src,
            body: InitBody::InitOk {
                id: node.next_id(),
                in_reply_to: id,
            },
        };
        //println!("Sending InitOk message...");
        digest.send_msg(resp);
        (digest, node)
    }

    pub async fn next_msg(&mut self) -> Message<N::Body> {
        match self.recv.as_mut() {
            None => read_msg(&mut self.stdin).await,
            Some(recv) => {
                tokio::select! {
                    msg = read_msg(&mut self.stdin) => msg,
                    msg = recv.recv() => msg.expect("node hung up (likely crashed)"),
                }
            }
        }
    }

    pub fn send_msg<B>(&mut self, msg: Message<B>)
    where
        B: MessageBody,
    {
        println!("{}", serde_json::to_string(&msg).expect(SER_ERR_MSG));
    }
}

async fn read_msg<B: MessageBody>(stdin: &mut Lines<BufReader<Stdin>>) -> Message<B> {
    loop {
        match stdin.next_line().await {
            Err(err) => {
                panic!("Error while reading line: {err}")
            }
            Ok(None) => continue,
            Ok(Some(line)) => {
                let val: OrInit<B> = serde_json::from_str(&line).expect(DE_ERR_MSG);
                match val.0 {
                    Either::Left(msg) => return msg,
                    Either::Right(Message {
                        body: InitBody::InitOk { .. },
                        ..
                    }) => continue,
                    Either::Right(Message {
                        body: InitBody::Init { .. },
                        ..
                    }) => panic!("mutliple init messages recieved"),
                }
            }
        }
    }
}
