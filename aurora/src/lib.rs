use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod client;
mod ids;
mod message;
mod node;
pub use client::*;
pub use ids::*;
pub use message::*;
pub use node::*;

pub trait RequestBody: Serialize + DeserializeOwned + Debug + Clone {
    type Response: ResponseBody<Request = Self>;

    fn into_response(self, msg_id: MessageId) -> Self::Response;
}

pub trait ResponseBody: Serialize + DeserializeOwned + Debug + Clone {
    type Request: RequestBody<Response = Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename = "init")]
pub struct InitRequest {
    #[serde(rename = "msg_id")]
    pub id: Option<MessageId>,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename = "init_ok")]
pub struct InitResponse {
    #[serde(rename = "msg_id")]
    pub id: MessageId,
    pub in_reply_to: Option<MessageId>,
}

impl RequestBody for InitRequest {
    type Response = InitResponse;

    fn into_response(self, id: MessageId) -> Self::Response {
        InitResponse {
            id,
            in_reply_to: self.id,
        }
    }
}

impl ResponseBody for InitResponse {
    type Request = InitRequest;
}

#[cfg(test)]
mod tests {
    use crate::{InitRequest, InitResponse, MessageId};

    #[test]
    fn init_deserialize() {
        let raw =
            r#"{"type": "init", "msg_id": 1, "node_id": "n3", "node_ids": ["n1", "n2", "n3"] }"#;
        let body: InitRequest =
            serde_json::from_str(raw).expect("Could not deserialize InitRequest");
        let known = InitRequest {
            id: Some(MessageId(1)),
            node_id: "n3".into(),
            node_ids: vec!["n1".into(), "n2".into(), "n3".into()],
        };
        assert_eq!(body, known);
    }

    #[test]
    fn init_serialize() {
        let resp = InitResponse {
            id: MessageId(0),
            in_reply_to: Some(MessageId(1)),
        };
        let json = serde_json::to_string(&resp).expect("Unable to serialize InitResponse");
        let known = r#"{"type":"init_ok","in_reply_to":1}"#;
        assert_eq!(json, known);
    }
}
