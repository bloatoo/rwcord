use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RateLimits {
    buckets: Arc<Mutex<HashMap<String, Bucket>>>,
}
