use rwcord::Client;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    let client = Client::new(token);
    client.start().await.unwrap();
}
