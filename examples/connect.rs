use rwcord::{async_trait, discord::Message, http::HTTPClient, Client, Handler};

struct EventHandler {}

#[async_trait]
impl Handler for EventHandler {
    async fn on_message_create(message: Message, http: Box<HTTPClient>) {
        if message.content() == "Hey!" {
            let content = format!("Hey {}", message.author().username());
            message.reply(http, &content).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    let client = Client::new(token);
    client.start::<EventHandler>().await.unwrap();
}
