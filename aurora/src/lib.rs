use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod client;
mod message;
mod node;
pub use client::*;
pub use message::*;
pub use node::*;

pub trait MessageBody: Serialize + DeserializeOwned + Debug + Clone + PartialEq + Eq {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum InitBody {
    #[serde(rename = "init")]
    Init {
        #[serde(rename = "msg_id")]
        id: Option<MessageId>,
        node_id: String,
        node_ids: Vec<String>,
    },
    #[serde(rename = "init_ok")]
    InitOk {
        #[serde(rename = "msg_id")]
        id: MessageId,
        in_reply_to: Option<MessageId>,
    },
}

impl MessageBody for InitBody {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum EchoBody {
    #[serde(rename = "echo")]
    Echo {
        #[serde(rename = "msg_id")]
        id: MessageId,
        echo: String,
    },
    #[serde(rename = "echo_ok")]
    EchoOk {
        echo: String,
        msg_id: MessageId,
        in_reply_to: MessageId,
    },
}

impl MessageBody for EchoBody {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum IdBody {
    #[serde(rename = "generate")]
    Generate,
    #[serde(rename = "generate_ok")]
    GenerateOk { id: String },
}

impl MessageBody for IdBody {}
