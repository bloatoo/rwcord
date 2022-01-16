use serde_json::json;

pub mod message;
pub use message::Message;

pub const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=9&encoding=json";
pub const API_URL: &str = "https://discord.com/api/v9";

pub enum Payload {
    Heartbeat,
    Identify(String, u64),
}

impl ToString for Payload {
    fn to_string(&self) -> String {
        match self {
            Self::Heartbeat => r#"{"op":1,"d":null}"#.to_string(),
            Self::Identify(token, intents) => serde_json::to_string(&json!({
                "op": 2,
                "d": {
                    "token": token,
                    "intents": intents,
                    "properties": {
                        "$os": "linux",
                        "$browser": "rwcord",
                        "$device": "rwcord"
                    }
                }
            }))
            .unwrap(),
        }
    }
}

pub enum EventType {
    MessageCreate,
    GuildCreate,
    Ready,
    Unknown,
}

impl<T: AsRef<str>> From<T> for EventType {
    fn from(data: T) -> Self {
        use EventType::*;

        match data.as_ref() {
            "MESSAGE_CREATE" => MessageCreate,
            "GUILD_CREATE" => GuildCreate,
            "READY" => Ready,
            _ => Unknown,
        }
    }
}
