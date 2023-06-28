use super::object::Object;
use crate::error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize)]
pub struct Request<'a> {
    pub model: Model,
    pub input: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: &'a Option<String>,
}

impl<'a> Request<'a> {
    pub fn new(model: Model, input: &'a str, user: &'a Option<String>) -> Self {
        Self { model, input, user }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub object: Object,
    pub data: Vec<Data>,
    pub model: Model,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub object: Object,
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename(serialize = "text-embedding-ada-002"))]
    #[serde(rename(deserialize = "text-embedding-ada-002-v2"))]
    TextEmbeddingAda002,
}

impl FromStr for Model {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text-embedding-ada-002" => Ok(Self::TextEmbeddingAda002),
            _ => Err(Self::Err::UnsupportedModel(s.to_string())),
        }
    }
}

// text-babbage-001, text-ada-001

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub total_tokens: usize,
}
