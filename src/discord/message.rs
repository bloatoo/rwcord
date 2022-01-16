use crate::http::HTTPClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    content: String,
    channel_id: String,
}

impl Message {
    /// The content of the message.
    pub fn content(&self) -> &String {
        &self.content
    }

    pub async fn reply(&self, http: Box<HTTPClient>, content: &str) {
        http.send_message(&self.channel_id, content).await.unwrap();
    }
}
