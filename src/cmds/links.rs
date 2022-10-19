use super::print_embedded_urls;
use egg_mode::{tweet::Tweet, Response};
use url::{Host, Url};

pub fn invoke(feed: &Response<Vec<Tweet>>, host: &Option<String>) {
    let filter = |url: &Url| {
        return if let Some(required_host) = host {
            if let Some(Host::Domain(url_host)) = url.host() {
                required_host == url_host
            } else {
                false
            }
        } else {
            true
        };
    };
    print_embedded_urls(feed.iter(), filter);
}
