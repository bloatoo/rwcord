// Credit to https://github.com/xyaman for his implementation of dealing with rate limiting at
// https://github.com/xyaman/panda

use reqwest::Response;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Default)]
pub struct Bucket {
    pub limit: u32,
    pub remaining: u32,
    pub reset: u64,
}

#[derive(Default)]
pub struct RateLimits {
    buckets: Arc<Mutex<HashMap<String, Bucket>>>,
}

impl RateLimits {
    pub async fn check(&self, key: &str) {
        let mut buckets_lock = self.buckets.lock().await;

        if let Some(bucket) = buckets_lock.get_mut(key) {
            let current = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let difference = Duration::from_secs(bucket.reset.saturating_sub(current));

            if difference.as_secs() == 0 {
                bucket.remaining += 1;
            } else if bucket.remaining == 0 {
                sleep(difference).await;
                bucket.remaining = bucket.limit;
            }

            bucket.remaining -= 1;
        }
    }

    pub async fn update(&self, key: String, response: &Response) {
        let headers = response.headers();
        let mut buckets_lock = self.buckets.lock().await;

        let bucket = buckets_lock.entry(key).or_default();

        if let Some(val) = headers.get("x-ratelimit-limit") {
            bucket.limit = val.to_str().unwrap().parse::<u32>().unwrap();
        }

        if let Some(val) = headers.get("x-ratelimit-remaining") {
            bucket.remaining = val.to_str().unwrap().parse::<u32>().unwrap();
        }

        if let Some(val) = headers.get("x-ratelimit-reset") {
            bucket.reset = val.to_str().unwrap().parse::<u64>().unwrap();
        }
    }
}
