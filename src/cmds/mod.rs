pub mod images;
pub mod links;
pub mod videos;

use egg_mode::tweet::Tweet;
use std::slice::Iter;
use url::Url;

fn print_embedded_urls<F>(iterator: Iter<'_, Tweet>, filter: F)
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
