use rwcord::{
    async_trait,
    discord::{Message, User},
    Client, Context, Handler,
};

struct EventHandler {}

#[derive(Clone)]
struct State {
    pub self_user: User,
}

#[async_trait]
impl Handler<State> for EventHandler {
    async fn on_ready(ctx: Context<State>, self_user: User) {
        let mut state = ctx.state().write().await;
        state.self_user = self_user;
    }

    async fn on_message_create(ctx: Context<State>, message: Message) {
        if message.content() == "Hey!" {
            let state = ctx.state().read().await;

            let content = format!(
                "Hey {}, my ID is {}",
                message.author().username(),
                state.self_user.id(),
            );

            if let Err(e) = message.reply(ctx.http(), &content).await {
                println!("Failed sending message: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();

    let s = State {
        self_user: User::blank(),
    };

    let client = Client::new(token);
    client.start::<EventHandler>(s).await.unwrap();
}
