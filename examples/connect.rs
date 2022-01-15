use rwcord::connect;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    connect(token).await.unwrap();
}
