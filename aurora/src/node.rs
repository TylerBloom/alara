use crate::{Client, InitRequest, Message, MessageId, RequestBody};

pub trait Node: Sized {
    type Body: RequestBody;

    fn init(msg: &Message<InitRequest>) -> Self;

    fn next_id(&mut self) -> MessageId;

    fn handle_msg(
        &mut self,
        client: &mut Client<Self>,
        msg: Message<Self::Body>,
    ) -> anyhow::Result<Option<Message<Self::Body>>>;
}
