use crate::discord::{Message, API_URL};
use reqwest::{Client, Response};
use serde_json::json;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct HTTPClient {
    client: Box<Client>,
    token: &'static str,
}

impl HTTPClient {
    pub fn new(token: &'static str) -> Self {
        let client = Box::new(Client::new());
        Self { client, token }
    }

    pub async fn get(&self, path: &str) -> Result<Response, Box<dyn Error>> {
        let res = self
            .client
            .get(API_URL.to_owned() + path)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bot {}", self.token))
            .send()
            .await?;

        Ok(res)
    }

    pub async fn post(&self, path: &str, body: String) -> Result<Response, Box<dyn Error>> {
        let res = self
            .client
            .post(API_URL.to_owned() + path)
            .body(body)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bot {}", self.token))
            .send()
            .await?;

        Ok(res)
    }

    pub async fn send_message(
        &self,
        channel_id: &str,
        content: &str,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let path = format!("/channels/{}/messages", channel_id);

        let body = serde_json::to_string(&json!({
            "content": content,
            "tts": false,
        }))
        .unwrap();

        let res = self.post(&path, body).await?;
        let res_json = res.text().await?;
        let message = serde_json::from_str(&res_json)?;

        Ok(message)
    }
}
