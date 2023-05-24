use anyhow::Error;
use async_trait::async_trait;
use openai_api::Datasource;
use std::sync;
use structopt::StructOpt;

use super::command::Command;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(StructOpt)]
pub enum Subcommand {
    List,
}

#[async_trait]
impl Command for Opt {
    async fn run(
        &self,
        http_client: sync::Arc<reqwest::Client>,
        api_key: String,
    ) -> Result<(), Error> {
        let datasource = openai_api::OpenAIApi::new(http_client, api_key);
        let files = datasource.list_files().await?;

        println!("{:#?}", files);

        Ok(())
    }
}
