use tokio::sync::mpsc::UnboundedSender;

use crate::{Message, MessageBody, MessageId};

/// The main trait which is used to model a node.
pub trait Node: Sized {
    /// The message type that this node expects to communicate
    type Body: MessageBody;

    /// Creates a new node from the data contained in an `Init` message
    fn init(
        sender: UnboundedSender<Message<Self::Body>>,
        node_id: String,
        node_ids: Vec<String>,
    ) -> Self;

    /// Retrieves the next `MessageId` from the node.
    ///
    /// NOTE: Nodes should never create the same message id twice.
    fn next_id(&mut self) -> MessageId;

    /// The main method used to process messages that the client receives. This method is not
    /// required to return response message.
    ///
    /// NOTE: Messages that the client receives via the channel are not passed through this method.
    fn handle_msg(
        &mut self,
        msg: Message<Self::Body>,
    ) -> anyhow::Result<Option<Message<Self::Body>>>;
}
