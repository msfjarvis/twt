use egg_mode::{entities::VideoVariant, tweet::Tweet, Response};
use std::slice::Iter;

pub fn invoke(feed: Response<Vec<Tweet>>) {
    print_video_urls(feed.iter());
}

// It'd be significantly easier to just use [Vec::sort_by] but lol, lmao.
// (It's actually because we don't have a mutable reference to the Vec).
fn find_largest_video(videos: &Vec<VideoVariant>) -> &VideoVariant {
    let mut largest: Option<&VideoVariant> = None;
    for video in videos {
        if let Some(bitrate) = video.bitrate {
            if largest.is_none()
                || (largest.is_some() && bitrate > largest.unwrap().bitrate.unwrap())
            {
                largest = Some(video);
            }
        }
    }
    largest.unwrap()
}

fn print_video_urls(iterator: Iter<'_, Tweet>) {
    iterator
        .filter_map(|status| status.extended_entities.as_ref())
        .flat_map(|entities| &entities.media)
        .flat_map(|x| &x.video_info)
        .map(|x| &x.variants)
        .filter(|variants| !variants.is_empty())
        .map(find_largest_video)
        .for_each(|x| println!("{}", x.url));
}
