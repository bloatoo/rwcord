use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Bucket {
    pub limit: u32,
    pub remaining: u32,
    pub reset: u32,
}

pub struct RateLimits {
    buckets: Arc<Mutex<HashMap<String, Bucket>>>,
}
