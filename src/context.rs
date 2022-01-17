use crate::http::HTTPClient;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Context<T>
where
    T: Clone,
{
    state: Arc<RwLock<T>>,
    http_client: Box<HTTPClient>,
}

impl<T> Context<T>
where
    T: Clone,
{
    pub fn new(state: T, token_box: Box<String>) -> Self {
        let state = Arc::new(RwLock::new(state));
        let token_leak = Box::leak(token_box);
        let http_client = Box::new(HTTPClient::new(token_leak));

        Self { state, http_client }
    }

    pub fn http(&self) -> &Box<HTTPClient> {
        &self.http_client
    }

    pub fn state(&self) -> &Arc<RwLock<T>> {
        &self.state
    }
}
