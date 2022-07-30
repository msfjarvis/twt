use std::slice::Iter;

use clap::{AppSettings, Args, Parser, Subcommand};
use color_eyre::Result;
use egg_mode::user::UserID;
use egg_mode::KeyPair;
use egg_mode::Token::Access;
use egg_mode::{entities::VideoVariant, tweet};
use mime::Mime;
use url::{Host, Url};

const CONSUMER_KEY: &str = std::env!("CONSUMER_KEY");
const CONSUMER_KEY_SECRET: &str = std::env!("CONSUMER_KEY_SECRET");
const ACCESS_TOKEN: &str = std::env!("ACCESS_TOKEN");
const ACCESS_TOKEN_SECRET: &str = std::env!("ACCESS_TOKEN_SECRET");
const ACCEPTED_MIME_TYPES: [Mime; 2] = [mime::IMAGE_JPEG, mime::IMAGE_PNG];

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
/// Fetches the last tweets of a given account, then prints original quality URLs for all image tweets.
struct CliOptions {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Images(Images),
    #[clap(arg_required_else_help = true)]
    Links(Links),
    #[clap(arg_required_else_help = true)]
    Videos(Videos),
}

#[derive(Debug, Args)]
/// Fetch original quality images from the tweets of a given Twitter user
struct Images {
    /// The Twitter username of the account to fetch images from.
    #[clap(long)]
    username: String,

    /// The maximum amount of tweets to check for images.
    #[clap(long, default_value = "1024")]
    max_amount: i32,

    /// Include retweets.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    with_rts: bool,

    /// Include replies.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    with_replies: bool,
}

#[derive(Debug, Args)]
struct Links {
    /// The Twitter username of the account to fetch links from.
    #[clap(long)]
    username: String,

    /// The host name to filter links on.
    #[clap(long, default_value = "imgur.com")]
    host: String,

    /// The maximum amount of tweets to check for images.
    #[clap(long, default_value = "1024")]
    max_amount: i32,

    /// Include retweets.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    with_rts: bool,

    /// Include replies.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    with_replies: bool,
}

#[derive(Debug, Args)]
struct Videos {
    /// The Twitter username of the account to fetch links from.
    #[clap(long)]
    username: String,

    /// The host name to filter links on.
    #[clap(long, default_value = "imgur.com")]
    host: String,

    /// The maximum amount of tweets to check for images.
    #[clap(long, default_value = "1024")]
    max_amount: i32,

    /// Include retweets.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    with_rts: bool,

    /// Include replies.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    with_replies: bool,
}

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
            let filter = |url: &Url| {
                return if let Some(segment) = url.path().split('/').last() {
                    let guess = mime_guess::from_path(segment);
                    guess
                        .first()
                        .filter(|mime| ACCEPTED_MIME_TYPES.contains(mime))
                        .is_some()
                } else {
                    false
                };
            };
            print_embedded_urls(feed.iter(), filter);
            print_media_urls(feed.iter());
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
            print_video_urls(feed.iter());
        }
        Commands::Videos(opts) => {
            let user_id: UserID = opts.username.into();

            let timeline = tweet::user_timeline(user_id, opts.with_replies, opts.with_rts, &token)
                .with_page_size(opts.max_amount);
            let (_, feed) = timeline.start().await?;
            print_video_urls(feed.iter());
        }
    }

    Ok(())
}

// It'd be significantly easier to just use [Vec::sort_by] but lol, lmao.
// (It's actually because we don't have a mutable reference to the Vec).
fn find_largest_video(videos: &Vec<VideoVariant>) -> &VideoVariant {
    let mut largest: Option<&VideoVariant> = None;
    for video in videos {
        if let Some(bitrate) = video.bitrate {
            if largest.is_none()
                || (largest.is_some() && bitrate > largest.unwrap().bitrate.unwrap())
            {
                largest = Some(video);
            }
        }
    }
    largest.unwrap()
}

fn print_video_urls(iterator: Iter<'_, tweet::Tweet>) {
    iterator
        .filter_map(|status| status.extended_entities.as_ref())
        .flat_map(|entities| &entities.media)
        .flat_map(|x| &x.video_info)
        .map(|x| &x.variants)
        .filter(|variants| !variants.is_empty())
        .map(find_largest_video)
        .for_each(|x| println!("{}", x.url));
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
