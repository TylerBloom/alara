use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct MessageId(pub usize);

impl From<usize> for MessageId {
    fn from(value: usize) -> Self {
        MessageId(value)
    }
}
