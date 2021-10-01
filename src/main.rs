use std::slice::Iter;

use color_eyre::Result;
use egg_mode::tweet;
use egg_mode::user::UserID;
use egg_mode::KeyPair;
use egg_mode::Token::Access;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    rename_all = "kebab-case",
    about = "Fetches the last tweets of a given account, then prints original quality URLs for all image tweets."
)]
struct CliOptions {
    /// The Twitter username of the account to fetch images from.
    #[structopt(env)]
    username: String,

    /// The maximum amount of tweets to check for images.
    #[structopt(long, default_value = "1024")]
    max_amount: i32,

    /// The consumer API key for the project.
    #[structopt(long, env)]
    consumer_key: String,

    /// The consumer key secret for the project.
    #[structopt(long, env)]
    consumer_key_secret: String,

    /// The access token for your user, for the project.
    #[structopt(long, env)]
    access_token: String,

    /// The access token secret for your user.
    #[structopt(long, env)]
    access_token_secret: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let options: CliOptions = CliOptions::from_args();

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

fn print_urls(iterator: Iter<tweet::Tweet>) {
    iterator
        .filter_map(|status| status.entities.media.as_ref())
        .flatten()
        .map(|x| &x.media_url_https)
        .filter(|x| !x.contains("thumb"))
        .for_each(|x| println!("{}:orig", x))
}
