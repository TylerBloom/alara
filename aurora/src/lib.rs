//! This library contains everything common amoung the different fly.io distributed systems
//! challenege problems.
//!
//! Include are implementations of all the message types, an trait to model a `Node` in the
//! cluster, and a client to abstract over the interface between of stdin and stdout. The
//! implementators of solutions to the problems should only need to concern themselves the internal
//! logic of a node.
//!
//! This problem takes an opinionated approach to modeling nodes and their communication by making
//! most of the interfaces `async` and with a preference for using things in the `tokio`
//! ecoecosystem.

#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    missing_docs,
    unused,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub,
    unsafe_code,
)]

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod client;
mod message;
mod node;

pub use client::*;
pub use message::*;
pub use node::*;

/// A super trait to create a shorthand for all the traits that a message body needs as they are
/// used as bounds in lots of places.
pub trait MessageBody: Serialize + DeserializeOwned + Debug + Clone + PartialEq + Eq {}

/// The message body type used to establish a node
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum InitBody {
    /// This data is passed to the `init` method of the `Node` trait to construct the `Node`.
    #[serde(rename = "init")]
    Init {
        /// The message id
        msg_id: Option<MessageId>,
        /// The `Node`'s id
        node_id: String,
        /// The ids of all other `Node` in the network
        node_ids: Vec<String>,
    },
    /// This data communicates the the `Node` was succeesfully established
    #[serde(rename = "init_ok")]
    InitOk {
        /// The message id
        msg_id: MessageId,
        /// The id of the message that this is responding to
        in_reply_to: Option<MessageId>,
    },
}

impl MessageBody for InitBody {}

/// The message body type used to in the echo problem
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum EchoBody {
    /// The data that communicates an echo action
    #[serde(rename = "echo")]
    Echo {
        /// The message id
        msg_id: MessageId,
        /// The message needs to be echoed back
        echo: String,
    },
    /// The data returned by the node as part of the echo response
    #[serde(rename = "echo_ok")]
    EchoOk {
        /// The message that is being echoed back
        echo: String,
        /// The message id
        msg_id: MessageId,
        /// The id of the message that this is responding to
        in_reply_to: MessageId,
    },
}

impl MessageBody for EchoBody {}

/// The message body type used to in the unique id generation problem
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum IdBody {
    /// The data that communicates a unique id generation action
    #[serde(rename = "generate")]
    Generate,
    /// The data returned by the node as part of the unique id generation response
    #[serde(rename = "generate_ok")]
    GenerateOk {
        /// The id that was generated
        id: String
    },
}

impl MessageBody for IdBody {}
