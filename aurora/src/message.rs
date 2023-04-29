use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::MessageBody;

#[derive(
    Serialize, Deserialize, Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct MessageId(pub usize);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(bound = "B: MessageBody")]
pub struct Message<B: MessageBody> {
    pub src: String,
    pub dest: String,
    pub body: B,
}

impl<B: MessageBody> Message<B> {
    pub fn into_response<F>(&mut self, f: F)
    where
        F: FnOnce(&mut B),
    {
        std::mem::swap(&mut self.src, &mut self.dest);
        f(&mut self.body)
    }
}

impl From<usize> for MessageId {
    fn from(value: usize) -> Self {
        MessageId(value)
    }
}
