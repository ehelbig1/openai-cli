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
    #[structopt(long, short, default_value = "text-davinci-003")]
    pub model: openai_api::model::create_completion::Model,

    pub prompt: String,

    #[structopt(long, short)]
    pub suffix: Option<String>,

    #[structopt(long, default_value="100")]
    pub max_tokens: usize,

    #[structopt(long, short, default_value="0.0")]
    pub temperature: f32,
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
            Subcommand::Create(opt) => openai_api::model::create_completion::Request::new(
                opt.model.clone(),
                opt.prompt.clone(),
            )
            .max_tokens(opt.max_tokens)
            .temperature(opt.temperature),
        };

        let response = datasource.create_completion(&request).await?;

        println!("{:#?}", response);

        Ok(())
    }
}
