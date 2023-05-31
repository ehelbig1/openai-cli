use super::object::Object;
use crate::error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub struct Request {
    pub model: Model,
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Request {
    pub fn new(model: Model, prompt: String) -> Self {
        Self {
            model,
            prompt,
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }

    pub fn suffix(mut self, suffix: String) -> Self {
        self.suffix = Some(suffix);
        self
    }

    pub fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        if temperature <= 2.0 || temperature >= 0.0 {
            self.temperature = Some(temperature);
        }

        self
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: Object,
    pub created: usize,
    pub model: Model,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<usize>,
    pub fishish_reason: Option<FinishReason>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "text-davinci-003")]
    TextDavinci003,

    #[serde(rename = "text-davinci-002")]
    TextDavinci002,

    #[serde(rename = "text-curie-001")]
    TextCurie001,

    #[serde(rename = "text-babbage-001")]
    TextBabbage001,

    #[serde(rename = "text-ada-001")]
    TextAda001,
}

impl FromStr for Model {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text-davinci-003" => Ok(Self::TextDavinci003),
            "text-davinci-002" => Ok(Self::TextDavinci002),
            "text-curie-001" => Ok(Self::TextCurie001),
            "text-babbage-001" => Ok(Self::TextBabbage001),
            "text-ada-001" => Ok(Self::TextAda001),
            _ => Err(Self::Err::UnsupportedModel(s.to_string())),
        }
    }
}

// text-babbage-001, text-ada-001

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Deserialize)]
pub enum FinishReason {
    #[serde(rename = "length")]
    Length,

    #[serde(rename = "stop")]
    Stop,
}
