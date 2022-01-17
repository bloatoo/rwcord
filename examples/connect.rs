use rwcord::{async_trait, discord::Message, http::HTTPClient, Client, Handler};

struct EventHandler {}

#[derive(Clone)]
struct State {}

#[async_trait]
impl Handler<State> for EventHandler {
    async fn on_message_create(message: Message, http: Box<HTTPClient>, _state: State) {
        if message.content() == "Hey!" {
            let content = format!("Hey {}", message.author().username());
            message.reply(http, &content).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    let s = State {};
    let client = Client::new(token, s);
    client.start::<EventHandler>().await.unwrap();
}
