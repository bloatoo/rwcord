use rwcord::{Client, Handler};

struct EventHandler {}

impl Handler for EventHandler {}

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    let client = Client::new(token);
    client.start::<EventHandler>().await.unwrap();
}
