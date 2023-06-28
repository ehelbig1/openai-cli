use super::function::Function;
use super::object::Object;
use crate::error;
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Serialize)]
pub struct Request<'a> {
    pub model: &'a Model,
    pub messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,

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

impl<'a> Request<'a> {
    pub fn new(model: &'a Model, messages: Vec<Message>) -> Self {
        Self {
            model,
            messages,
            functions: None,
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

    pub fn functions(mut self, functions: Vec<Function>) -> Self {
        self.functions = Some(functions);
        self
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FunctionCall {
    name: String,
    arguments: Option<HashMap<String, serde_json::Value>>,
}

impl<'de> Deserialize<'de> for FunctionCall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            name: String,
            arguments: Option<String>,
        }

        let helper = Inner::deserialize(deserializer)?;
        let arguments_map: Option<HashMap<String, serde_json::Value>> = helper
            .arguments
            .map(|args| serde_json::from_str(&args))
            .transpose()
            .map_err(serde::de::Error::custom)?;

        let arguments = match arguments_map {
            Some(ref m) if m.is_empty() => None,
            _ => arguments_map,
        };

        Ok(FunctionCall {
            name: helper.name,
            arguments,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
    Function,
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
    pub finish_reason: FinishReason,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "gpt-4")]
    Gpt4,

    #[serde(rename = "gpt-4-0613")]
    Gpt4_0613,

    #[serde(rename = "gpt-4-32k")]
    Gpt4_32k,

    #[serde(rename = "gpt-4-32k-0613")]
    Gpt4_32k_0613,

    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3dot5Turbo,

    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt3dot5Turbo_16k,

    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt3dot5Turbo_0613,

    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    Gpt3dot5Turbo_16k_0613,
}

impl FromStr for Model {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4-0613" => Ok(Self::Gpt4_0613),
            "gpt-4-32k" => Ok(Self::Gpt4_32k),
            "gpt-4-32k-0613" => Ok(Self::Gpt4_32k_0613),
            "gpt-3.5-turbo" => Ok(Self::Gpt3dot5Turbo),
            "gpt-3.5-turbo-16k" => Ok(Self::Gpt3dot5Turbo_16k),
            "gpt-3.5-turbo-0613" => Ok(Self::Gpt3dot5Turbo_0613),
            "gpt-3.5-turbo-16k-0613" => Ok(Self::Gpt3dot5Turbo_16k_0613),
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

    #[serde(rename = "function_call")]
    FunctionCall,
}
