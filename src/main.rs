mod cli;
mod cmds;

use crate::cli::{CliOptions, Commands};
use crate::cmds::{images, videos};
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
            let filter = |url: &Url| {
                return if let Some(Host::Domain(h)) = url.host() {
                    opts.host == h
                } else {
                    false
                };
            };
            print_embedded_urls(feed.iter(), filter);
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

fn print_embedded_urls<F>(iterator: Iter<'_, tweet::Tweet>, filter: F)
where
    F: FnMut(&Url) -> bool,
{
    iterator
        .map(|status| &status.entities)
        .flat_map(|entities| &entities.urls)
        .filter_map(|url| url.expanded_url.as_ref())
        .flat_map(|url| Url::parse(url))
        .filter(filter)
        .for_each(|url| println!("{url}"));
}
