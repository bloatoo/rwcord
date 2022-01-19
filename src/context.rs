use crate::http::HTTPClient;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Context<T> {
    state: Arc<RwLock<T>>,
    http_client: Box<HTTPClient>,
}

impl<T> Context<T> {
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

impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            http_client: self.http_client.clone(),
        }
    }
}
