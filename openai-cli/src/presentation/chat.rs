use anyhow::Error;
use async_trait::async_trait;
use console;
use openai_api::Datasource;
use std::{io, sync};
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
    #[structopt(long, short, default_value = "gpt-3.5-turbo")]
    pub model: openai_api::model::create_chat::Model,

    #[structopt(long)]
    pub max_tokens: Option<usize>,

    #[structopt(long, short)]
    pub temperature: Option<f32>,
}

#[async_trait]
impl Command for Opt {
    async fn run(
        &self,
        http_client: sync::Arc<reqwest::Client>,
        api_key: String,
    ) -> Result<(), Error> {
        let datasource = openai_api::OpenAIApi::new(http_client, api_key);

        let messages: Vec<openai_api::model::create_chat::Message> =
            vec![openai_api::model::create_chat::Message {
                role: openai_api::model::create_chat::Role::System,
                content: String::from("You are a very helpful assistant"),
                name: None,
            }];

        let mut request = match &self.subcommand {
            Subcommand::Create(opt) => {
                openai_api::model::create_chat::Request::new(opt.model.clone(), messages)
            }
        };

        let assistant_response = console::Style::new().blue();
        let assistant = console::Style::new().dim();

        println!(
            "{}",
            assistant_response.apply_to("What can I assist you with?")
        );

        loop {
            let mut content = String::new();

            io::stdin()
                .read_line(&mut content)
                .expect("Failed to read input");

            request
                .messages
                .push(openai_api::model::create_chat::Message {
                    role: openai_api::model::create_chat::Role::User,
                    content,
                    name: None,
                });

            let response = datasource.create_chat(&request).await?;

            println!(
                "{:#?}: {}",
                assistant.apply_to(&response.choices[0].message.role),
                assistant_response.apply_to(&response.choices[0].message.content)
            );

            request.messages.push(response.choices[0].message.clone());

            println!()
        }
    }
}
