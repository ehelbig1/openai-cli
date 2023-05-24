use crate::error;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3dot5turbo,

    #[serde(rename = "gpt-4")]
    Gpt4,

    #[serde(rename = "text-davinci-003")]
    TextDavinci003,
}

impl FromStr for Model {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-3.5-turbo" => Ok(Self::Gpt3dot5turbo),
            "gpt-4" => Ok(Self::Gpt4),
            "text-davinci-003" => Ok(Self::TextDavinci003),
            _ => Err(Self::Err::UnsupportedModel(s.to_string())),
        }
    }
}
