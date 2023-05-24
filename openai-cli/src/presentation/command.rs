use anyhow::Error;
use async_trait::async_trait;
use std::sync;

#[async_trait]
pub trait Command {
    async fn run(
        &self,
        http_client: sync::Arc<reqwest::Client>,
        api_key: String,
    ) -> Result<(), Error>;
}
