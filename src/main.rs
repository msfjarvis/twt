use std::slice::Iter;

use color_eyre::Result;
use egg_mode::tweet;
use egg_mode::user::UserID;
use egg_mode::KeyPair;
use egg_mode::Token::Access;

#[tokio::main]
async fn main() -> Result<()> {
    let consumer_key = std::env::var("CONSUMER_KEY")?;
    let consumer_secret = std::env::var("CONSUMER_KEY_SECRET")?;
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let access_token_secret = std::env::var("ACCESS_TOKEN_SECRET")?;
    let username = std::env::var("TARGET_USERNAME")?;

    let consumer = KeyPair::new(consumer_key, consumer_secret);
    let access = KeyPair::new(access_token, access_token_secret);
    let token = Access { consumer, access };

    let user_id: UserID = username.into();

    let timeline = tweet::user_timeline(user_id, false, false, &token).with_page_size(1024);
    let (_, feed) = timeline.start().await?;
    print_urls(feed.iter());

    Ok(())
}

fn print_urls(iterator: Iter<tweet::Tweet>) {
    for status in iterator {
        if let Some(entities) = &status.entities.media {
            for item in entities {
                println!("{}:orig", item.media_url_https);
            }
        }
    }
}
