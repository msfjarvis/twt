use std::slice::Iter;

use clap::{AppSettings, Parser};
use color_eyre::Result;
use egg_mode::tweet;
use egg_mode::user::UserID;
use egg_mode::KeyPair;
use egg_mode::Token::Access;
use mime::Mime;
use url::Url;

const ACCEPTED_MIME_TYPES: [Mime; 2] = [mime::IMAGE_JPEG, mime::IMAGE_PNG];

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

    #[clap(long, default_value_t = false, parse(try_from_str))]
    with_rts: bool,

    #[clap(long, default_value_t = false, parse(try_from_str))]
    with_replies: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = CliOptions::parse();

    let consumer = KeyPair::new(options.consumer_key, options.consumer_key_secret);
    let access = KeyPair::new(options.access_token, options.access_token_secret);
    let token = Access { consumer, access };

    let user_id: UserID = options.username.into();

    let timeline = tweet::user_timeline(user_id, options.with_replies, options.with_rts, &token)
        .with_page_size(options.max_amount);
    let (_, feed) = timeline.start().await?;
    print_embedded_urls(feed.iter());
    print_media_urls(feed.iter());

    Ok(())
}

fn print_media_urls(iterator: Iter<'_, tweet::Tweet>) {
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

fn print_embedded_urls(iterator: Iter<'_, tweet::Tweet>) {
    let mut urls = iterator
        .map(|status| &status.entities)
        .map(|entities| &entities.urls)
        .flatten()
        .filter_map(|url| url.expanded_url.as_ref())
        .flat_map(|url| Url::parse(url))
        .collect::<Vec<Url>>();
    urls.dedup();
    for url in urls {
        if let Some(segment) = url.path().split("/").last() {
            let guess = mime_guess::from_path(segment);
            if let Some(_) = guess
                .first()
                .filter(|mime| ACCEPTED_MIME_TYPES.contains(mime))
            {
                println!("{url}");
            }
        };
    }
}
