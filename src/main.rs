use std::slice::Iter;

use clap::{AppSettings, Parser};
use color_eyre::Result;
use egg_mode::tweet;
use egg_mode::user::UserID;
use egg_mode::KeyPair;
use egg_mode::Token::Access;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
/// Fetches the last tweets of a given account, then prints original quality URLs for all image tweets.
struct CliOptions {
    /// The Twitter username of the account to fetch images from.
    #[clap(env = "TARGET_USERNAME")]
    username: String,

    /// The maximum amount of tweets to check for images.
    #[clap(long, default_value = "1024")]
    max_amount: i32,

    /// The consumer API key for the project.
    #[clap(long, env, default_value = std::option_env!("CONSUMER_KEY").unwrap_or(""))]
    consumer_key: String,

    /// The consumer key secret for the project.
    #[clap(long, env, default_value = std::option_env!("CONSUMER_KEY_SECRET").unwrap_or(""))]
    consumer_key_secret: String,

    /// The access token for your user, for the project.
    #[clap(long, env, default_value = std::option_env!("ACCESS_TOKEN").unwrap_or(""))]
    access_token: String,

    /// The access token secret for your user.
    #[clap(long, env, default_value = std::option_env!("ACCESS_TOKEN_SECRET").unwrap_or(""))]
    access_token_secret: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = CliOptions::parse();

    let consumer = KeyPair::new(options.consumer_key, options.consumer_key_secret);
    let access = KeyPair::new(options.access_token, options.access_token_secret);
    let token = Access { consumer, access };

    let user_id: UserID = options.username.into();

    let timeline =
        tweet::user_timeline(user_id, false, false, &token).with_page_size(options.max_amount);
    let (_, feed) = timeline.start().await?;
    print_urls(feed.iter());

    Ok(())
}

fn print_urls(iterator: Iter<'_, tweet::Tweet>) {
    let mut urls = iterator
        .filter_map(|status| status.extended_entities.as_ref())
        .flat_map(|entities| &entities.media)
        .map(|x| &x.media_url_https)
        .filter(|x| !x.contains("thumb"))
        .collect::<Vec<&String>>();
    urls.dedup();
    for url in urls {
        println!("{url}:orig");
    }
}
