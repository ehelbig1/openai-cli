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
    Create(Create),
}

#[derive(StructOpt)]
pub struct Create {
    #[structopt(long, short, default_value = "text-embedding-ada-002")]
    pub model: openai_api::model::create_embedding::Model,

    pub input: String,

    #[structopt(long, short)]
    pub user: Option<String>,
}

#[async_trait]
impl Command for Opt {
    async fn run(
        &self,
        http_client: sync::Arc<reqwest::Client>,
        api_key: String,
    ) -> Result<(), Error> {
        let datasource = openai_api::OpenAIApi::new(http_client, api_key);

        let request = match &self.subcommand {
            Subcommand::Create(opt) => openai_api::model::create_embedding::Request::new(
                opt.model.clone(),
                &opt.input,
                &opt.user,
            ),
        };

        let response = datasource.create_embedding(&request).await?;

        println!("{:#?}", response);

        Ok(())
    }
}
