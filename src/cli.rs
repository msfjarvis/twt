use clap::{AppSettings, Args, Parser, Subcommand};
use egg_mode::{
    tweet::{self, Timeline},
    user::UserID,
    Token,
};

pub trait TimelineCreator {
    fn create_timeline(&self, token: Token) -> Timeline;
}

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
/// Fetches the last tweets of a given account, then prints original quality URLs for all image tweets.
pub struct Opts {
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

impl TimelineCreator for Images {
    fn create_timeline(&self, token: Token) -> Timeline {
        let user_id: UserID = self.username.clone().into();

        tweet::user_timeline(user_id, self.with_replies, self.with_rts, &token)
            .with_page_size(self.max_amount)
    }
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

impl TimelineCreator for Links {
    fn create_timeline(&self, token: Token) -> Timeline {
        let user_id: UserID = self.username.clone().into();

        tweet::user_timeline(user_id, self.with_replies, self.with_rts, &token)
            .with_page_size(self.max_amount)
    }
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

impl TimelineCreator for Videos {
    fn create_timeline(&self, token: Token) -> Timeline {
        let user_id: UserID = self.username.clone().into();

        tweet::user_timeline(user_id, self.with_replies, self.with_rts, &token)
            .with_page_size(self.max_amount)
    }
}
