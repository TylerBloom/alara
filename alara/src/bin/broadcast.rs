#![allow(dead_code)]
use std::collections::HashSet;

use aurora::*;
use tokio::sync::mpsc::UnboundedSender;

#[tokio::main]
async fn main() {
    main_loop::<BroadcastNode>().await
}

struct BroadcastNode {
    id: String,
    counter: usize,
    messages: HashSet<usize>,
    //sender: UnboundedSender<Message<BroadcastBody>>,
}

impl Node for BroadcastNode {
    type Body = BroadcastBody;

    fn init(
        _sender: UnboundedSender<Message<Self::Body>>,
        node_id: String,
        _: Vec<String>,
    ) -> Self {
        Self {
            id: node_id,
            counter: 0,
            //sender,
            messages: HashSet::new(),
        }
    }

    fn next_id(&mut self) -> MessageId {
        let id = self.counter;
        MessageId(id)
    }

    fn handle_msg(
        &mut self,
        mut msg: Message<Self::Body>,
    ) -> anyhow::Result<Option<Message<Self::Body>>> {
        msg.into_response(|body| self.transform_msg_body(body));
        Ok(Some(msg))
    }
}

impl BroadcastNode {
    fn transform_msg_body(&mut self, body: &mut BroadcastBody) {
        match body {
            BroadcastBody::Broadcast { message, msg_id } => {
                self.messages.insert(*message);
                *body = BroadcastBody::BroadcastOk {
                    msg_id: self.next_id(),
                    in_reply_to: *msg_id,
                };
            }
            BroadcastBody::Read { msg_id } => {
                *body = BroadcastBody::ReadOk {
                    messages: self.messages.clone(),
                    msg_id: Some(self.next_id()),
                    in_reply_to: Some(*msg_id),
                };
            }
            BroadcastBody::Topology { msg_id, .. } => {
                *body = BroadcastBody::TopologyOk {
                    msg_id: self.next_id(),
                    in_reply_to: *msg_id,
                }
            }
            BroadcastBody::BroadcastOk { .. }
            | BroadcastBody::ReadOk { .. }
            | BroadcastBody::TopologyOk { .. } => {}
        }
    }
}
