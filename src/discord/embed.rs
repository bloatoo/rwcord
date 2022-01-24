use crate::discord::message::Sendable;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EmbedFooter {
    pub text: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: String,
    pub icon_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Embed {
    title: String,
    description: String,
    color: i32,
    url: String,
    footer: EmbedFooter,
    author: EmbedAuthor,
    fields: Vec<EmbedField>,
    image: EmbedImage,
}

impl Sendable for Embed {
    fn to_request_body(self) -> String {
        let embed_json: String = serde_json::to_string(&self).unwrap();

        println!("{:#?}", embed_json);

        let json = format!(r#"{{"embed":{},"tts":"false"}}"#, embed_json);

        println!("{}", json);
        json
    }
}

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title<T: ToString>(self, title: T) -> Self {
        Self {
            title: title.to_string(),
            ..self
        }
    }

    pub fn image(self, image: EmbedImage) -> Self {
        Self { image, ..self }
    }

    pub fn description<T: ToString>(self, description: T) -> Self {
        Self {
            description: description.to_string(),
            ..self
        }
    }

    pub fn color<T: ToString>(self, color: T) -> Self {
        let color = &color.to_string()[1..];

        let color = i32::from_str_radix(color, 16).unwrap();
        Self { color, ..self }
    }

    pub fn url<T: ToString>(self, url: T) -> Self {
        Self {
            url: url.to_string(),
            ..self
        }
    }

    pub fn footer(self, f: EmbedFooter) -> Self {
        Self { footer: f, ..self }
    }

    pub fn author(self, a: EmbedAuthor) -> Self {
        Self { author: a, ..self }
    }

    pub fn add_field(self, f: EmbedField) -> Self {
        let mut fields = self.fields;
        fields.push(f);

        Self { fields, ..self }
    }
}
