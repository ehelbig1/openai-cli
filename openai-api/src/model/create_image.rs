use crate::error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url;

#[derive(Debug, Serialize)]
pub struct Request {
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Request {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            n: None,
            size: None,
            response_format: None,
            user: None,
        }
    }

    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }
}

#[derive(Debug, Serialize)]
pub enum Size {
    #[serde(rename = "256x256")]
    _256x256_,

    #[serde(rename = "512x512")]
    _512x512_,

    #[serde(rename = "1024x1024")]
    _1024x1024_,
}

impl FromStr for Size {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "256x256" => Ok(Self::_256x256_),
            "512x512" => Ok(Self::_512x512_),
            "1024x1024" => Ok(Self::_1024x1024_),
            _ => Err(Self::Err::UnsupportedImageSize(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum ResponseFormat {
    Url,

    #[serde(rename = "b64_json")]
    B64Json,
}

impl FromStr for ResponseFormat {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "url" => Ok(Self::Url),
            "b64json" => Ok(Self::B64Json),
            _ => Err(error::Error::UnsupportedResponseFormat(s.to_string())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub created: usize,
    pub data: Vec<ImageUrl>,
}

#[derive(Debug, Deserialize)]
pub struct ImageUrl {
    pub url: Option<url::Url>,
    pub b64_json: Option<String>,
}
