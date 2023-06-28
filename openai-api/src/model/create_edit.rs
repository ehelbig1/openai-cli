use std::str::FromStr;
use crate::error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    pub model: Model,
    pub input: String,
    pub instruction: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<usize>
}

impl Request {
    pub fn new(model: Model, input: String, instruction: String) -> Self {
        Self {
            model,
            input,
            instruction,
            n: None,
            temperature: None,
            top_p: None
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum Model {
    #[serde(rename = "text-davinci-edit-001")]
    TextDavinciEdit001,

    #[serde(rename = "code-davinci-edit-001")]
    CodeDavinciEdit001,
}

impl FromStr for Model {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text-davinci-edit-001" => Ok(Self::TextDavinciEdit001),
            "code-davinci-code-001" => Ok(Self::CodeDavinciEdit001),
            _ => Err(Self::Err::UnsupportedModel(s.to_string())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub object: Object,
    pub created: usize,
    pub choices: Vec<Choice>,
    pub usage: Usage
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: usize
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize
}

#[derive(Debug, Deserialize)]
pub enum Object {
    #[serde(rename = "edit")]
    Edit
}