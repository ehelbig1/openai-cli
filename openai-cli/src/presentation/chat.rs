use anyhow::Error;
use async_trait::async_trait;
use console;
use openai_api::Datasource;
use std::{io, sync, collections::HashMap};
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
    #[structopt(long, short, default_value = "gpt-3.5-turbo-0613")]
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
                content: Some(String::from("You are a very helpful assistant")),
                name: None,
                function_call: None,
            }];

        // let functions = vec![openai_api::model::function::Function::new(
        //     String::from("test"),
        //     String::from("this function is great for testing"),
        // )
        // .add_property(
        //     String::from("test"),
        //     openai_api::model::function::Parameter::String(
        //         openai_api::model::function::JsonString::new(Some(String::from("Test")), None),
        //     ),
        //     true,
        // )
        // .add_property(
        //     String::from("test2"),
        //     openai_api::model::function::Parameter::Object(
        //         openai_api::model::function::JsonObject::new(
        //             HashMap::from([
        //                 (
        //                     String::from("test"), openai_api::model::function::Parameter::Object(
        //                         openai_api::model::function::JsonObject::new(
        //                             HashMap::from([
        //                                 (
        //                                     String::from("test"),
        //                                     openai_api::model::function::Parameter::String(
        //                                         openai_api::model::function::JsonString::new(
        //                                             None,
        //                                             Some(vec![
        //                                                 String::from("this"),
        //                                                 String::from("is"),
        //                                                 String::from("a"),
        //                                                 String::from("test")
        //                                             ])
        //                                         )
        //                                     )
        //                                 )
        //                             ]),
        //                             vec![])
        //                     )
        //                 )
        //             ]),
        //             vec![]
        //         )
        //     ),
        //     true,
        // )];

        let mut request = match &self.subcommand {
            Subcommand::Create(opt) => {
                openai_api::model::create_chat::Request::new(&opt.model, messages)
                    // .functions(functions)
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
                    content: Some(content),
                    name: None,
                    function_call: None,
                });

            let response = datasource.create_chat(&request).await?;

            match response.choices[0].finish_reason {
                openai_api::model::create_chat::FinishReason::FunctionCall => {
                    println!(
                        "{:#?}: {:#?}",
                        assistant.apply_to(&response.choices[0].message.role),
                        assistant_response
                            .apply_to(response.choices[0].message.function_call.as_ref().unwrap())
                    );
                }
                _ => {
                    println!(
                        "{:#?}: {:#?}",
                        assistant.apply_to(&response.choices[0].message.role),
                        assistant_response
                            .apply_to(&response.choices[0].message.content.as_ref().unwrap())
                    );

                    request.messages.push(response.choices[0].message.clone());
                }
            }

            println!()
        }
    }
}
