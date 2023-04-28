use std::fmt::Debug;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{InitRequest, InitResponse, MessageId, RequestBody};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message<B: RequestBody> {
    pub src: String,
    pub dest: String,
    #[serde(deserialize_with = "message_deserialize_wrapper")]
    pub body: MessageBody<B>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageBody<B: RequestBody> {
    #[serde(deserialize_with = "request_deserialize_wrapper")]
    Main(B),
    MainOk(B::Response),
    Init(InitRequest),
    InitOk(InitResponse),
}

impl<B: RequestBody> Message<B> {
    pub fn into_response(self, msg_id: MessageId) -> Self {
        Message {
            src: self.dest,
            dest: self.src,
            body: self.body.into_response(msg_id),
        }
    }
}

impl<B: RequestBody> MessageBody<B> {
    pub fn into_response(self, msg_id: MessageId) -> Self {
        match self {
            MessageBody::Init(msg) => MessageBody::InitOk(msg.into_response(msg_id)),
            MessageBody::Main(msg) => MessageBody::MainOk(msg.into_response(msg_id)),
            MessageBody::InitOk(ok) => MessageBody::InitOk(ok),
            MessageBody::MainOk(ok) => MessageBody::MainOk(ok),
        }
    }
}

fn message_deserialize_wrapper<'de, B, D>(deserializer: D) -> Result<MessageBody<B>, D::Error>
where
    D: Deserializer<'de>,
    B: RequestBody,
{
    MessageBody::<B>::deserialize(deserializer)
}

fn request_deserialize_wrapper<'de, B, D>(deserializer: D) -> Result<B, D::Error>
where
    D: Deserializer<'de>,
    B: RequestBody,
{
    B::deserialize(deserializer)
}
