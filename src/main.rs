use serde_json::Value;
use std::error::Error;

use futures::channel::mpsc;
use futures::stream::StreamExt;

use tokio_tungstenite::connect_async;

const API_URL: &str = "wss://gateway.discord.gg/?v=9&encoding=json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sock, _) = connect_async(API_URL)
        .await
        .expect("Failed connecting to Discord");

    let (write, read) = sock.split();

    let mut read = read.fuse();

    loop {
        let msg = read.next().await.expect("Message was None").unwrap();
        let text = msg.to_text()?;

        let json: Value = serde_json::from_str(text)?;

        let opcode = json["op"].as_u64().unwrap();

        match opcode {
            _ => (),
        }
    }
}
