use super::object::Object;
use crate::error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize)]
pub struct Request {
    pub model: Model,
    pub messages: Vec<Message>,

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
    pub stop: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Request {
    pub fn new(model: Model, messages: Vec<Message>) -> Self {
        Self {
            model,
            messages,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }

    pub fn max_tokens(mut self, max_tokens: Option<usize>) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn temperature(mut self, temperature: Option<f32>) -> Self {
        if let Some(temperature) = temperature {
            if temperature <= 2.0 || temperature >= 0.0 {
                self.temperature = Some(temperature);
            }
        }

        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: Role,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}

impl FromStr for Role {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(Self::System),
            "assistant" => Ok(Self::Assistant),
            "user" => Ok(Self::User),
            _ => Err(Self::Err::UnsupportedRole(s.to_string())),
        }
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
    pub index: usize,
    pub message: Message,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "gpt-4")]
    Gpt4,

    #[serde(rename = "gpt-4-0314")]
    Gpt4_0314,

    #[serde(rename = "gpt-4-32k")]
    Gpt4_32k,

    #[serde(rename = "gpt-4-32k-0314")]
    Gpt4_32k0314,

    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3dot5Turbo,

    #[serde(rename = "gpt-3.5-turbo-0301")]
    Gpt3dot5Turbo0301,
}

impl FromStr for Model {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4-0314" => Ok(Self::Gpt4_0314),
            "gpt-4-32k" => Ok(Self::Gpt4_32k),
            "gpt-4-32k-0314" => Ok(Self::Gpt4_32k0314),
            "gpt-3.5-turbo" => Ok(Self::Gpt3dot5Turbo),
            "gpt-3.5-turbo-0301" => Ok(Self::Gpt3dot5Turbo0301),
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
