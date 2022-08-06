use clap::{AppSettings, Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
/// Fetches the last tweets of a given account, then prints original quality URLs for all image tweets.
pub struct CliOptions {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    Images(Images),
    #[clap(arg_required_else_help = true)]
    Links(Links),
    #[clap(arg_required_else_help = true)]
    Videos(Videos),
}

#[derive(Debug, Args)]
/// Fetch original quality images from the tweets of a given Twitter user
pub struct Images {
    /// The Twitter username of the account to fetch images from.
    #[clap(long)]
    pub username: String,

    /// The maximum amount of tweets to check for images.
    #[clap(long, default_value = "1024")]
    pub max_amount: i32,

    /// Include retweets.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_rts: bool,

    /// Include replies.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_replies: bool,
}

#[derive(Debug, Args)]
pub struct Links {
    /// The Twitter username of the account to fetch links from.
    #[clap(long)]
    pub username: String,

    /// The host name to filter links on.
    #[clap(long, default_value = "imgur.com")]
    pub host: String,

    /// The maximum amount of tweets to check for images.
    #[clap(long, default_value = "1024")]
    pub max_amount: i32,

    /// Include retweets.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_rts: bool,

    /// Include replies.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_replies: bool,
}

#[derive(Debug, Args)]
pub struct Videos {
    /// The Twitter username of the account to fetch links from.
    #[clap(long)]
    pub username: String,

    /// The host name to filter links on.
    #[clap(long, default_value = "imgur.com")]
    pub host: String,

    /// The maximum amount of tweets to check for images.
    #[clap(long, default_value = "1024")]
    pub max_amount: i32,

    /// Include retweets.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_rts: bool,

    /// Include replies.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_replies: bool,
}
