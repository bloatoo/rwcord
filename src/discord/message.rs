use super::User;
use crate::http::HTTPClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

pub trait Sendable {
    fn to_request_body(self) -> String;
}

impl<T> Sendable for T
where
    T: Into<String>,
{
    fn to_request_body(self) -> String {
        let content: String = self.into();

        serde_json::to_string(&json!({
            "content": content,
            "tts": false,
        }))
        .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    content: String,
    channel_id: String,
    author: User,
}

impl Message {
    /// Send a message to the channel this message was sent in.
    pub async fn reply(
        &self,
        http: &Box<HTTPClient>,
        content: impl Sendable,
    ) -> Result<Message, Box<dyn Error>> {
        Ok(http.send_message(&self.channel_id, content).await?)
    }

    /// The content of the message.
    pub fn content(&self) -> &String {
        &self.content
    }

    /// The ID of the channel the message was sent in.
    pub fn channel_id(&self) -> &String {
        &self.channel_id
    }

    /// The author of the message.
    pub fn author(&self) -> &User {
        &self.author
    }
}
