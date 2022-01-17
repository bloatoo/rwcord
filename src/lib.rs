use serde_json::Value;
use std::error::Error;

pub use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

use futures::{
    channel::mpsc::{self, Sender},
    select,
    sink::SinkExt,
    stream::StreamExt,
};

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

pub mod discord;
use discord::{EventType, Message, Payload, User, GATEWAY_URL};

pub mod http;

pub mod context;
pub use context::Context;

#[async_trait]
pub trait Handler<T>
where
    T: Clone + Send + Sync + 'static,
{
    async fn on_message_create(_ctx: Context<T>, _message: Message) {}
    async fn on_ready(_ctx: Context<T>, _self: User) {}
    async fn on_guild_create(_ctx: Context<T>) {}
}

pub struct Client<T> {
    state: T,
    token: String,
}

impl<T> Client<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(token: String, state: T) -> Self {
        Self { token, state }
    }

    pub async fn start<H: Handler<T>>(&self) -> Result<(), Box<dyn Error>> {
        let (sock, _) = connect_async(GATEWAY_URL)
            .await
            .expect("Failed connecting to Discord");

        let (mut write, read) = sock.split();
        let (heartbeat_tx, heartbeat_rx) = mpsc::channel::<u8>(1);

        let mut read = read.fuse();
        let mut heartbeat_rx = heartbeat_rx.fuse();

        let token_box = Box::new(self.token.clone());

        let context = Context::new(self.state.clone(), token_box.clone());

        loop {
            select! {
                msg = read.next() => {
                    match msg {
                        Some(msg) => {
                            let msg = msg.expect("Message contains an error");
                            let text = msg.to_text().unwrap();

                            if text.is_empty() { continue; }

                            let json: Value = serde_json::from_str(text).unwrap();
                            println!("{}", serde_json::to_string_pretty(&json).unwrap());

                            let opcode = json["op"].as_u64().unwrap();

                            match opcode {
                                10 => {
                                    let heartbeat_interval = json["d"]["heartbeat_interval"].as_u64().unwrap();
                                    let heartbeat_tx = heartbeat_tx.clone();

                                    spawn_heartbeater(heartbeat_interval, heartbeat_tx);

                                    let identify = TungsteniteMessage::Text(Payload::Identify(self.token.clone(), 513).to_string());
                                    write.send(identify).await.unwrap();
                                }
                                _ => (),
                            }

                            if let Some(t) = json["t"].as_str() {
                                use EventType::*;

                                let ctx = context.clone();

                                match EventType::from(t) {
                                    MessageCreate => {
                                        let message = serde_json::from_value(json["d"].clone()).unwrap();

                                        tokio::spawn(async move {
                                            H::on_message_create(ctx, message).await;
                                        });
                                    }

                                    Ready => {
                                        let user = serde_json::from_value(json["d"]["user"].clone()).unwrap();

                                        tokio::spawn(async move {
                                            H::on_ready(ctx, user).await;
                                        });
                                    }

                                    GuildCreate => {
                                        tokio::spawn(async move {
                                            H::on_guild_create(ctx).await;
                                        });
                                    }
                                    _ => ()
                                }
                            }
                        }
                        None => break,
                    }
                }

                _ = heartbeat_rx.next() => {
                    let message = TungsteniteMessage::Text(Payload::Heartbeat.to_string());
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
