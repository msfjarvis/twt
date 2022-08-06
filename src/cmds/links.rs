use super::print_embedded_urls;
use egg_mode::{tweet::Tweet, Response};
use url::{Host, Url};

pub fn invoke(feed: Response<Vec<Tweet>>, host: &str) {
    let filter = |url: &Url| {
        return if let Some(Host::Domain(h)) = url.host() {
            host == h
        } else {
            false
        };
    };
    print_embedded_urls(feed.iter(), filter);
}
