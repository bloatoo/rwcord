use rwcord::{
    async_trait,
    discord::{Message, User},
    http::HTTPClient,
    Client, Handler,
};

use std::sync::Arc;
use tokio::sync::Mutex;

struct EventHandler {}

#[derive(Clone)]
struct State {
    self_user: User,
}

#[async_trait]
impl Handler<Arc<Mutex<State>>> for EventHandler {
    async fn on_ready(self_user: User, _http: Box<HTTPClient>, state: Arc<Mutex<State>>) {
        let mut state = state.lock().await;
        state.self_user = self_user;
    }
    async fn on_message_create(message: Message, http: Box<HTTPClient>, _state: Arc<Mutex<State>>) {
        if message.content() == "Hey!" {
            let content = format!("Hey {}", message.author().username());
            message.reply(http, &content).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();

    let s = State {
        self_user: User::blank(),
    };

    let client = Client::new(token, Arc::new(Mutex::new(s)));
    client.start::<EventHandler>().await.unwrap();
}
