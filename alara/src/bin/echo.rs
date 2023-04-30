use aurora::*;
use tokio::sync::mpsc::UnboundedSender;

#[tokio::main]
async fn main() {
    main_loop::<EchoNode>().await
}

struct EchoNode {
    counter: usize,
}

impl Node for EchoNode {
    type Body = EchoBody;

    fn init(_: UnboundedSender<Message<Self::Body>>, _: String, _: Vec<String>) -> Self {
        Self { counter: 0 }
    }

    fn next_id(&mut self) -> MessageId {
        let id = self.counter;
        MessageId(id)
    }

    fn handle_msg(
        &mut self,
        mut msg: Message<Self::Body>,
    ) -> anyhow::Result<Option<Message<Self::Body>>> {
        match &msg.body {
            EchoBody::EchoOk { .. } => Ok(None),
            EchoBody::Echo { msg_id: id, echo } => {
                let echo = echo.clone();
                let in_reply_to = *id;
                let msg_id = self.next_id();
                msg.into_response(|body| {
                    *body = EchoBody::EchoOk {
                        echo,
                        msg_id,
                        in_reply_to,
                    }
                });
                Ok(Some(msg))
            }
        }
    }
}
