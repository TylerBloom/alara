use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::MessageBody;

/// A new-type wrapper around message ids.
#[derive(
    Serialize, Deserialize, Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct MessageId(pub usize);

/// The message type that is sent and recieved by the client.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(bound = "B: MessageBody")]
pub struct Message<B: MessageBody> {
    /// The identifier of the `Message`'s source
    pub src: String,
    /// The identifier of the `Message`'s destination
    pub dest: String,
    /// The body of the `Message`, contains most of the relative data to solution implementors
    pub body: B,
}

impl<B: MessageBody> Message<B> {
    /// Turns a request message into a response message by mutating the data in-place.
    /// The given function takes a mutable reference to the message's current body, allowing the
    /// function to mutate the body at will.
    ///
    /// NOTE: This method also swaps the `src` and `dest` fields of the messages. It is generally
    /// recommended to only call this method only once (or an odd number of times). Otherwise, the
    /// Maelstrom tests will fail.
    pub fn into_response<F>(&mut self, f: F)
    where
        F: FnOnce(&mut B),
    {
        std::mem::swap(&mut self.src, &mut self.dest);
        f(&mut self.body)
    }

    /// Creates a copy of the message but uses a new message id in the body
    pub fn clone_with_msg_id(&self, msg_id: MessageId) -> Self {
        let mut digest = self.clone();
        digest.body.update_msg_id(msg_id);
        digest
    }
}

impl From<usize> for MessageId {
    fn from(value: usize) -> Self {
        MessageId(value)
    }
}
