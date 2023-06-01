use anyhow::Error;
use std::{env, sync, time};
use structopt::StructOpt;

mod presentation;

use presentation::command::Command;

#[derive(StructOpt)]
struct Opt {
    // #[structopt(short, long)]
    // version: bool,
    #[structopt(long, env = "OPENAI_API_KEY", hide_env_values = true)]
    api_key: String,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    Model(presentation::model::Opt),
    Completion(presentation::completion::Opt),
    Chat(presentation::chat::Opt),
    Image(presentation::image::Opt),
    File(presentation::file::Opt),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let opt = Opt::from_args();
    let api_key = opt.api_key;

    // if opt.version {
    //     return println!("{}", env::var("CARGO_PKG_VERSION").unwrap());
    // }

    let http_client = sync::Arc::new(
        reqwest::Client::builder()
            .timeout(time::Duration::from_secs(64))
            .build()
            .expect("Creating HTTP Client"),
    );

    match opt.subcommand {
        Subcommand::Model(opt) => opt.run(http_client, api_key).await?,
        Subcommand::Completion(opt) => opt.run(http_client, api_key).await?,
        Subcommand::Chat(opt) => opt.run(http_client, api_key).await?,
        Subcommand::Image(opt) => opt.run(http_client, api_key).await?,
        Subcommand::File(opt) => opt.run(http_client, api_key).await?,
    }

    Ok(())
}
