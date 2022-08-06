mod cli;
mod cmds;

use crate::cli::{CliOptions, Commands};
use crate::cmds::{images, links, videos};
use clap::Parser;
use color_eyre::Result;
use egg_mode::tweet;
use egg_mode::user::UserID;
use egg_mode::KeyPair;
use egg_mode::Token::Access;
use std::slice::Iter;
use url::{Host, Url};

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
            let user_id: UserID = opts.username.into();

            let timeline = tweet::user_timeline(user_id, opts.with_replies, opts.with_rts, &token)
                .with_page_size(opts.max_amount);
            let (_, feed) = timeline.start().await?;
            images::invoke(feed);
        }
        Commands::Links(opts) => {
            let user_id: UserID = opts.username.into();

            let timeline = tweet::user_timeline(user_id, opts.with_replies, opts.with_rts, &token)
                .with_page_size(opts.max_amount);
            let (_, feed) = timeline.start().await?;
            links::invoke(feed, opts.host);
        }
        Commands::Videos(opts) => {
            let user_id: UserID = opts.username.into();

            let timeline = tweet::user_timeline(user_id, opts.with_replies, opts.with_rts, &token)
                .with_page_size(opts.max_amount);
            let (_, feed) = timeline.start().await?;
            videos::invoke(feed);
        }
    }

    Ok(())
}
