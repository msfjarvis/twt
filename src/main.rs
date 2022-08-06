mod cli;
mod cmds;

use crate::cli::{CliOptions, Commands, TimelineCreator};
use crate::cmds::{images, links, videos};
use clap::Parser;
use color_eyre::Result;
use egg_mode::KeyPair;
use egg_mode::Token::Access;

const CONSUMER_KEY: &str = std::env!("CONSUMER_KEY");
const CONSUMER_KEY_SECRET: &str = std::env!("CONSUMER_KEY_SECRET");
const ACCESS_TOKEN: &str = std::env!("ACCESS_TOKEN");
const ACCESS_TOKEN_SECRET: &str = std::env!("ACCESS_TOKEN_SECRET");

#[tokio::main]
async fn main() -> Result<()> {
    let options = CliOptions::parse();

    let consumer = KeyPair::new(CONSUMER_KEY, CONSUMER_KEY_SECRET);
    let access = KeyPair::new(ACCESS_TOKEN, ACCESS_TOKEN_SECRET);
    let token = Access { consumer, access };

    match options.command {
        Commands::Images(opts) => {
            let timeline = opts.create_timeline(token);
            let (_, feed) = timeline.start().await?;
            images::invoke(feed);
        }
        Commands::Links(opts) => {
            let host = opts.host.clone();
            let timeline = opts.create_timeline(token);
            let (_, feed) = timeline.start().await?;
            links::invoke(feed, &host);
        }
        Commands::Videos(opts) => {
            let timeline = opts.create_timeline(token);
            let (_, feed) = timeline.start().await?;
            videos::invoke(feed);
        }
    }

    Ok(())
}
