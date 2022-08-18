use clap::{AppSettings, Args, Parser, Subcommand};
use egg_mode::{
    tweet::{self, Timeline},
    user::UserID,
    Token,
};

pub trait TimelineCreator {
    fn timeline(&self, token: Token) -> Timeline;

    fn create_timeline(&self, opts: &CommonCliOpts, token: Token) -> Timeline {
        let user_id: UserID = opts.username.clone().into();

        tweet::user_timeline(user_id, opts.with_replies, opts.with_rts, &token)
            .with_page_size(opts.max_amount)
    }
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
pub struct CommonCliOpts {
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
/// Fetch original quality images from the tweets of a given Twitter user
pub struct Images {
    #[clap(flatten)]
    opts: CommonCliOpts,

    /// Include replies.
    #[clap(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_replies: bool,
}

impl TimelineCreator for Images {
    fn timeline(&self, token: Token) -> Timeline {
        self.create_timeline(&self.opts, token)
    }
}

#[derive(Debug, Args)]
pub struct Links {
    #[clap(flatten)]
    opts: CommonCliOpts,

    /// The host name to filter links on.
    #[clap(long, default_value = "imgur.com")]
    pub host: String,
}

impl TimelineCreator for Links {
    fn timeline(&self, token: Token) -> Timeline {
        self.create_timeline(&self.opts, token)
    }
}

#[derive(Debug, Args)]
pub struct Videos {
    #[clap(flatten)]
    opts: CommonCliOpts,

    /// The host name to filter links on.
    #[clap(long, default_value = "imgur.com")]
    pub host: String,
}

impl TimelineCreator for Videos {
    fn timeline(&self, token: Token) -> Timeline {
        self.create_timeline(&self.opts, token)
    }
}
