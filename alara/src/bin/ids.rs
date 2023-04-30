use aurora::*;

#[tokio::main]
async fn main() {
    main_loop::<IdsNode>().await
}

struct IdsNode {
    id: String,
    counter: usize,
}

impl Node for IdsNode {
    type Body = IdBody;

    fn init(
        _: tokio::sync::mpsc::UnboundedSender<Message<Self::Body>>,
        node_id: String,
        _: Vec<String>,
    ) -> Self {
        Self {
            id: node_id,
            counter: 0,
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
        match &msg.body {
            IdBody::GenerateOk { .. } => Ok(None),
            IdBody::Generate => {
                let id = self.next_id().0;
                msg.into_response(|body| {
                    *body = IdBody::GenerateOk {
                        id: format!("{}-{id}", self.id),
                    }
                });
                Ok(Some(msg))
            }
        }
    }
}
