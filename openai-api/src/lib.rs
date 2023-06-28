use async_trait::async_trait;
use std::sync;

mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async fn list_models(&self) -> Result<model::list_models::Response, error::Error>;
    async fn create_completion(
        &self,
        request: &model::create_completion::Request,
    ) -> Result<model::create_completion::Response, error::Error>;
    async fn create_chat(
        &self,
        request: &model::create_chat::Request,
    ) -> Result<model::create_chat::Response, error::Error>;
    async fn create_image(
        &self,
        request: &model::create_image::Request,
    ) -> Result<model::create_image::Response, error::Error>;
    async fn create_edit(
        &self,
        request: &model::create_edit::Request,
    ) -> Result<model::create_edit::Response, error::Error>;
    async fn list_files(&self) -> Result<model::list_files::Response, error::Error>;
    async fn create_embedding(
        &self,
        request: &model::create_embedding::Request,
    ) -> Result<model::create_embedding::Response, error::Error>;
}

pub struct OpenAIApi {
    http_client: sync::Arc<reqwest::Client>,
    api_key: String,
    base_url: String,
}

impl OpenAIApi {
    pub fn new(http_client: sync::Arc<reqwest::Client>, api_key: String) -> Self {
        Self {
            http_client,
            api_key,
            base_url: String::from("https://api.openai.com"),
        }
    }
}

#[async_trait]
impl Datasource for OpenAIApi {
    async fn list_models(&self) -> Result<model::list_models::Response, error::Error> {
        let response = self
            .http_client
            .get(format!("{}/v1/models", &self.base_url))
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(response) => {
                let data: model::list_models::Response = response.json().await?;

                Ok(data)
            }
            Err(error) => Err(error::Error::InvalidHttpResponse(error.to_string())),
        }
    }

    async fn create_completion(
        &self,
        request: &model::create_completion::Request,
    ) -> Result<model::create_completion::Response, error::Error> {
        let body = serde_json::to_string(&request)?;

        let response = self
            .http_client
            .post(format!("{}/v1/completions", &self.base_url))
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .body(body)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(response) => {
                let data: model::create_completion::Response = response.json().await?;

                Ok(data)
            }
            Err(error) => Err(error::Error::InvalidHttpResponse(error.to_string())),
        }
    }

    async fn create_chat(
        &self,
        request: &model::create_chat::Request,
    ) -> Result<model::create_chat::Response, error::Error> {
        let body = serde_json::to_string(&request)?;

        let response = self
            .http_client
            .post(format!("{}/v1/chat/completions", &self.base_url))
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .body(body)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(response) => {
                let data: model::create_chat::Response = response.json().await?;

                Ok(data)
            }
            Err(error) => Err(error::Error::InvalidHttpResponse(error.to_string())),
        }
    }

    async fn create_image(
        &self,
        request: &model::create_image::Request,
    ) -> Result<model::create_image::Response, error::Error> {
        let body = serde_json::to_string(&request)?;

        let response = self
            .http_client
            .post(format!("{}/v1/images/generations", &self.base_url))
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .body(body)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(response) => {
                let data: model::create_image::Response = response.json().await?;

                Ok(data)
            }
            Err(error) => Err(error::Error::InvalidHttpResponse(error.to_string())),
        }
    }

    async fn create_edit(
        &self,
        request: &model::create_edit::Request,
    ) -> Result<model::create_edit::Response, error::Error> {
        let body = serde_json::to_string(&request)?;

        let response = self
            .http_client
            .post(format!("{}/v1/edits", &self.base_url))
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .body(body)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(response) => {
                let data: model::create_edit::Response = response.json().await?;

                Ok(data)
            }
            Err(error) => Err(error::Error::InvalidHttpResponse(error.to_string())),
        }
    }

    async fn list_files(&self) -> Result<model::list_files::Response, error::Error> {
        let response = self
            .http_client
            .get(format!("{}/v1/files", &self.base_url))
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(response) => {
                let data: model::list_files::Response = response.json().await?;

                Ok(data)
            }
            Err(error) => Err(error::Error::InvalidHttpResponse(error.to_string())),
        }
    }

    async fn create_embedding(
        &self,
        request: &model::create_embedding::Request,
    ) -> Result<model::create_embedding::Response, error::Error> {
        let body = serde_json::to_string(&request)?;

        let response = self
            .http_client
            .post(format!("{}/v1/embeddings", &self.base_url))
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .body(body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }
}
