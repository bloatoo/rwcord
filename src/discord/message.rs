use super::User;
use crate::http::HTTPClient;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
        content: &str,
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
