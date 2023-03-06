mod cli;
mod cmds;
mod config;

use crate::cli::{Commands, Opts, TimelineCreator};
#[cfg(feature = "videos")]
use crate::cmds::videos;
use crate::cmds::{images, links};
use crate::config::Credentials;
use clap::Parser;
use color_eyre::Result;
use egg_mode::KeyPair;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = crate::config::get_path()?;
    let config_str = std::fs::read_to_string(config_path.as_path())?;
    let credentials: Credentials = toml::from_str(&config_str)?;
    let options = Opts::parse();

    let con_token = KeyPair::new(credentials.consumer_key, credentials.consumer_key_secret);
    let token = egg_mode::auth::bearer_token(&con_token).await?;

    match options.command {
        Commands::Images(opts) => {
            let timeline = opts.timeline(token);
            let (_, feed) = timeline.start().await?;
            let feed = feed.iter();
            images::invoke(feed);
        }
        Commands::Links(opts) => {
            let host = opts.host.clone();
            let timeline = opts.timeline(token);
            let (_, feed) = timeline.start().await?;
            let feed = feed.iter();
            links::invoke(feed, &host);
        }
        #[cfg(feature = "videos")]
        Commands::Videos(opts) => {
            let timeline = opts.timeline(token);
            let (_, feed) = timeline.start().await?;
            let feed = feed.iter();
            videos::invoke(feed);
        }
    }

    Ok(())
}
