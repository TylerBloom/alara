use tokio::sync::mpsc::UnboundedSender;

use crate::{Message, MessageBody, MessageId};

pub trait Node: Sized {
    type Body: MessageBody;

    fn init(
        sender: UnboundedSender<Message<Self::Body>>,
        node_id: String,
        node_ids: Vec<String>,
    ) -> Self;

    fn next_id(&mut self) -> MessageId;

    fn handle_msg(
        &mut self,
        msg: Message<Self::Body>,
    ) -> anyhow::Result<Option<Message<Self::Body>>>;
}
