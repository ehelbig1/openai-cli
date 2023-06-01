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
    pub prompt: String,

    #[structopt(short, long = "number")]
    pub n: Option<usize>,

    #[structopt(short, long)]
    pub size: Option<openai_api::model::create_image::Size>,

    #[structopt(short, long)]
    pub response_format: Option<openai_api::model::create_image::ResponseFormat>,
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
            Subcommand::Create(opt) => {
                openai_api::model::create_image::Request::new(opt.prompt.clone())
            }
        };

        let response = datasource.create_image(&request).await?;

        println!("{}", response.data[0].url.as_ref().unwrap());

        Ok(())
    }
}
