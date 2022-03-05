use rwcord::{
    async_trait,
    discord::{
        embed::{Embed, EmbedAuthor, EmbedField, EmbedFooter},
        Message, User,
    },
    Client, Context, Handler,
};

struct EventHandler {}

#[async_trait]
impl Handler<()> for EventHandler {
    async fn on_message_create(ctx: Context<()>, message: Message) {
        if message.content() == "!embed" {
            let embed = Embed::new()
                .title("This is a test embed")
                .description("This is a description.")
                .color("#FF0000")
                .footer(EmbedFooter {
                    text: "This is a footer".into(),
                    ..Default::default()
                })
                .author(EmbedAuthor {
                    name: "Author".into(),
                    ..Default::default()
                })
                .add_field(EmbedField {
                    name: "This is an embed field".into(),
                    value: "This is the field's value".into(),
                    inline: true,
                })
                .add_field(EmbedField {
                    name: "This is another embed field".into(),
                    value: "This is the field's value".into(),
                    inline: true,
                });

            if let Err(e) = message.reply(ctx.http(), embed).await {
                println!("Failed sending message: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();

    let client = Client::new(token);
    client.start::<EventHandler>(()).await.unwrap();
}
