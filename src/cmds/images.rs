use super::print_embedded_urls;
use egg_mode::tweet::Tweet;
use mime::Mime;
use std::slice::Iter;
use url::Url;

const ACCEPTED_MIME_TYPES: [Mime; 2] = [mime::IMAGE_JPEG, mime::IMAGE_PNG];

// `feed` is passed over into two separate methods which consume the value fully.
#[allow(clippy::needless_pass_by_value)]
pub fn invoke(feed: Iter<Tweet>) {
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
    print_embedded_urls(feed.clone(), filter);
    print_media_urls(feed.clone());
}

fn print_media_urls(iterator: Iter<'_, Tweet>) {
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
