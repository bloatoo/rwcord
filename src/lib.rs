use serde_json::{json, Value};
use std::error::Error;

use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

use futures::{
    channel::mpsc::{self, Sender},
    select,
    sink::SinkExt,
    stream::StreamExt,
};

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub const API_URL: &str = "wss://gateway.discord.gg/?v=9&encoding=json";

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

#[async_trait]
pub trait Handler {
    async fn on_message_create() {}
    async fn on_ready() {}
    async fn on_guild_create() {}
}

pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub async fn start<H: Handler>(&self) -> Result<(), Box<dyn Error>> {
        let (sock, _) = connect_async(API_URL)
            .await
            .expect("Failed connecting to Discord");

        let (mut write, read) = sock.split();
        let (heartbeat_tx, heartbeat_rx) = mpsc::channel::<u8>(1);

        let mut read = read.fuse();
        let mut heartbeat_rx = heartbeat_rx.fuse();

        loop {
            select! {
                msg = read.next() => {
                    match msg {
                        Some(msg) => {
                            let msg = msg.expect("Message contains an error");
                            let text = msg.to_text().unwrap();

                            let json: Value = serde_json::from_str(text).unwrap();

                            let opcode = json["op"].as_u64().unwrap();

                            println!("{}", serde_json::to_string_pretty(&json).unwrap());

                            match opcode {
                                10 => {
                                    let heartbeat_interval = json["d"]["heartbeat_interval"].as_u64().unwrap();
                                    let heartbeat_tx = heartbeat_tx.clone();

                                    spawn_heartbeater(heartbeat_interval, heartbeat_tx);

                                    let identify = Message::Text(Payload::Identify(self.token.clone(), 513).to_string());
                                    write.send(identify).await.unwrap();
                                }
                                _ => (),
                            }

                            if let Some(t) = json["t"].as_str() {
                                use EventType::*;

                                match EventType::from(t) {
                                    MessageCreate => {
                                        H::on_message_create().await;
                                    }
                                    Ready => {
                                        H::on_ready().await;
                                    }
                                    GuildCreate => { H::on_guild_create().await; }
                                    _ => ()
                                }
                            }
                        }
                        None => break,
                    }
                }

                _ = heartbeat_rx.next() => {
                    let message = Message::Text(Payload::Heartbeat.to_string());
                    write.send(message).await.unwrap();
                }
            }
        }

        Ok(())
    }
}

fn spawn_heartbeater(interval: u64, mut sender: Sender<u8>) {
    tokio::spawn(async move {
        sender.send(1).await.unwrap();
        sleep(Duration::from_millis(interval)).await;
    });
}
