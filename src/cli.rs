use clap::{Args, Parser, Subcommand};
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
#[command(author, version, about)]
/// Fetches the last tweets of a given account, then prints original quality URLs for all image tweets.
pub struct Opts {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Images(Images),
    #[command(arg_required_else_help = true)]
    Links(Links),
    #[command(arg_required_else_help = true)]
    Videos(Videos),
}

#[derive(Debug, Args)]
pub struct CommonCliOpts {
    /// The Twitter username of the account to fetch images from.
    #[arg(long)]
    pub username: String,

    /// The maximum amount of tweets to check for images.
    #[arg(long, default_value = "1024")]
    pub max_amount: i32,

    /// Include retweets.
    #[arg(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_rts: bool,

    /// Include replies.
    #[arg(long, default_value_t = false, value_parser = clap::value_parser!(bool))]
    pub with_replies: bool,
}

#[derive(Debug, Args)]
/// Fetch original quality images from the tweets of a given Twitter user
pub struct Images {
    #[command(flatten)]
    opts: CommonCliOpts,
}

impl TimelineCreator for Images {
    fn timeline(&self, token: Token) -> Timeline {
        self.create_timeline(&self.opts, token)
    }
}

#[derive(Debug, Args)]
/// Fetch links from the user's tweets, filtered based on the given host.
pub struct Links {
    #[command(flatten)]
    opts: CommonCliOpts,

    /// The host name to filter links on. Prints all links if no value is passed.
    #[arg(long)]
    pub host: Option<String>,
}

impl TimelineCreator for Links {
    fn timeline(&self, token: Token) -> Timeline {
        self.create_timeline(&self.opts, token)
    }
}

#[derive(Debug, Args)]
/// Fetch high quality versions of videos embedded in the user's tweets.
pub struct Videos {
    #[command(flatten)]
    opts: CommonCliOpts,
}

impl TimelineCreator for Videos {
    fn timeline(&self, token: Token) -> Timeline {
        self.create_timeline(&self.opts, token)
    }
}

#[cfg(test)]
mod test {
    use super::Opts;

    #[test]
    fn cli_assert() {
        <Opts as clap::CommandFactory>::command().debug_assert()
    }
}
