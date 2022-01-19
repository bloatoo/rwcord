use crate::discord::message::Sendable;

pub enum Request {
    SendMessage(Box<dyn Sendable>),
}
