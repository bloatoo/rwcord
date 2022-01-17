use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
}

impl User {
    /// The ID of the user.
    pub fn id(&self) -> &String {
        &self.id
    }

    /// The username of the user.
    pub fn username(&self) -> &String {
        &self.username
    }

    /// The discriminator ("tag") of the user.
    pub fn discriminator(&self) -> &String {
        &self.discriminator
    }

    pub fn blank() -> Self {
        Self {
            id: "1234".into(),
            username: "blank".into(),
            discriminator: "1234".into(),
        }
    }
}
