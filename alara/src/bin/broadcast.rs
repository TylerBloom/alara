#![allow(dead_code, unused)]
#![allow(clippy::expect_fun_call)]

use std::collections::{HashMap, HashSet};

use aurora::*;
use tokio::sync::mpsc::UnboundedSender;

#[tokio::main]
async fn main() {
    main_loop::<BroadcastNode>().await
}

const SENDER_UNWRAP: &str = "expected for sender over channel to succeed";

/// Approach:
/// This is taking an immediate-mode approach to gossiping. That is, as soon as a message is
/// received, outbound messages are sent to our adjecents. We then wait for a `BroadcastOk` message
/// to be received. If no such message is recieved for ... some amount of time, we resend that
/// broadcast message.
///
/// NOTE: In a real-world usecase, we would also want to try and detect if a node in our
/// district has gone down so that we can update our district and/or report the problem.
/// This isn't a concern here as Maelstrom will shut everything down in that case, but this is
/// still worth keeping in mind.
#[derive(Debug)]
struct BroadcastNode {
    id: String,
    counter: MessageIdCounter,
    messages: HashSet<usize>,
    sender: UnboundedSender<Message<BroadcastBody>>,
    // The ids of adjecents mapped to the messages we know they have
    adjecents: HashMap<String, Adjecent>,
}

#[derive(Debug, Default, PartialEq)]
struct Adjecent {
    // The set of values that we know that this node contains
    known: HashSet<usize>,
    // A map of the outbound messages for each adjecent node. Message values are moved to the inner
    // map to the main adjecents map when the appropriate `BroadcastOk` message is recieved.
    //
    // TODO: We need to track suspected dropped messages in such a way that we can automatically
    // resend those messages *and* cancel resending specific messages.
    pending: HashMap<MessageId, usize>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct MessageIdCounter {
    counter: usize,
}

impl Node for BroadcastNode {
    type Body = BroadcastBody;

    fn init(
        sender: UnboundedSender<Message<Self::Body>>,
        node_id: String,
        nodes: Vec<String>,
    ) -> Self {
        let adjecents = HashMap::with_capacity(nodes.len());
        Self {
            id: node_id,
            counter: MessageIdCounter { counter: 0 },
            sender,
            messages: HashSet::new(),
            adjecents,
        }
    }

    fn next_id(&mut self) -> MessageId {
        self.counter.next_id()
    }

    // What needs to happen when a broadcast message is recieved:
    //  Message value is added to our messages
    //   - If it was an unknown value, create a broadcast message for each adjacent node that doesn't
    //   contain the
    //
    //  Return `BroadcastOk`
    //
    // Broadcast forwarding:
    //
    fn handle_msg(
        &mut self,
        mut msg: Message<Self::Body>,
    ) -> anyhow::Result<Option<Message<Self::Body>>> {
        eprintln!("processing message: {msg:?}");
        match &mut msg.body {
            BroadcastBody::Broadcast { msg_id, message } => {
                let msg_id = *msg_id;
                let message = *message;
                self.handle_broadcast(&msg, msg_id, message);
                msg.into_response(|body| {
                    *body = BroadcastBody::BroadcastOk {
                        msg_id: self.next_id(),
                        in_reply_to: msg_id,
                    }
                });
                Ok(Some(msg))
            }
            BroadcastBody::BroadcastOk {
                msg_id,
                in_reply_to,
            } => {
                self.handle_broadcast_ok(&msg.src, *in_reply_to);
                Ok(None)
            }
            BroadcastBody::Read { msg_id } => {
                let msg_id = *msg_id;
                msg.into_response(|body| {
                    *body = BroadcastBody::ReadOk {
                        msg_id: Some(self.next_id()),
                        in_reply_to: Some(msg_id),
                        messages: self.messages.clone(),
                    }
                });
                Ok(Some(msg))
            }
            BroadcastBody::ReadOk {
                msg_id,
                in_reply_to,
                messages,
            } => Ok(None),
            BroadcastBody::Topology { msg_id, topology } => {
                let msg_id = *msg_id;
                self.handle_topology(topology);
                msg.into_response(|body| {
                    *body = BroadcastBody::TopologyOk {
                        msg_id: self.next_id(),
                        in_reply_to: msg_id,
                    }
                });
                Ok(Some(msg))
            }
            BroadcastBody::TopologyOk { .. } => Ok(None),
        }
    }
}

impl BroadcastNode {
    fn is_adjacent(&self, node: &String) -> bool {
        self.adjecents.contains_key(node)
    }

    /// Start propagating message
    fn handle_broadcast(
        &mut self,
        msg: &Message<BroadcastBody>,
        msg_id: MessageId,
        message: usize,
    ) {
        self.messages.insert(message);
        self.adjecents
            .iter_mut()
            .filter_map(|(n, adj)| {
                adj.add_message(&mut self.counter, message)
                    .map(|body| (n.clone(), body))
            })
            .map(|(dest, body)| Message {
                src: self.id.clone(),
                dest: format!("n{}", dest.strip_prefix('n').unwrap()),
                body,
            })
            .for_each(|msg| {
                let json = serde_json::to_string(&msg).unwrap();
                eprintln!("forwarding broadcast message: {json}");
                self.sender.send(msg).unwrap();
            })
    }

    /// Confirm the message has been propagated
    fn handle_broadcast_ok(&mut self, src: &String, msg_id: MessageId) {
        if let Some(adj) = self.adjecents.get_mut(src) {
            adj.update_pending(msg_id);
        }
    }

    fn handle_topology(&mut self, topology: &mut HashMap<String, HashSet<String>>) {
        self.adjecents.extend(
            topology
                .remove(&self.id)
                .expect(&format!("node {} was not in topology", self.id))
                .into_iter()
                .map(|n| (n, Adjecent::default())),
        );
    }
}

impl Adjecent {
    /// Attempts to add a message to the adjecent node's pending queue. Succeeds only if the
    /// message doesn't exist in the known values. On success, a broadcast body is returned.
    /// Otherwise, `None` is returned.
    fn add_message(
        &mut self,
        counter: &mut MessageIdCounter,
        message: usize,
    ) -> Option<BroadcastBody> {
        (!self.known.contains(&message)).then(|| {
            let msg_id = counter.next_id();
            self.pending.insert(msg_id, message);
            BroadcastBody::Broadcast { msg_id, message }
        })
    }

    fn update_pending(&mut self, msg_id: MessageId) {
        self.pending
            .remove(&msg_id)
            .map(|val| self.known.insert(val));
    }
}

impl MessageIdCounter {
    fn next_id(&mut self) -> MessageId {
        let id = self.counter;
        self.counter += 1;
        MessageId(id)
    }
}
