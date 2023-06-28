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
    #[structopt(long, short, default_value = "text-davinci-edit-001")]
    pub model: openai_api::model::create_edit::Model,

    pub input: String,
    pub instruction: String,

    #[structopt(long, short, default_value = "0.0")]
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
            Subcommand::Create(opt) => openai_api::model::create_edit::Request::new(
                opt.model.clone(),
                opt.input.clone(),
                opt.instruction.clone(),
            ),
        };

        let response = datasource.create_edit(&request).await?;

        println!("{}", response.choices[0].text);

        Ok(())
    }
}
