use crate::discord::message::Sendable;
use reqwest::Method;

pub enum RequestType<'a> {
    MessageSend(&'a dyn Sendable, String),
}
pub struct Request {
    method: Method,
}

impl Request {
    pub fn bucket_id(&self) {}

    pub fn build_req() {}
}
