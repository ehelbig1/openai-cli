use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Invalid HTTP response: {0}")]
    InvalidHttpResponse(String),

    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),

    #[error("Unsupported role: {0}")]
    UnsupportedRole(String),

    #[error("Unsupported image size: {0}")]
    UnsupportedImageSize(String),

    #[error("Unsupported response format: {0}")]
    UnsupportedResponseFormat(String),

    #[error("Json Serialization: {0}")]
    JsonSerialization(String),

    #[error("Reqwest: {0}")]
    Reqwest(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonSerialization(err.to_string())
    }
}
