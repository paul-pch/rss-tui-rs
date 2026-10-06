use feed_rs::{
    model::{Feed, FeedType},
    parser::ParseFeedResult,
};

use crate::config::Stream;
use std::error::Error;

struct Entry {
    title: String,
    author: String,
    updated: String,
    link: String,
}

struct _Feed {
    url: String,
    title: String,
    updated: String,
    entries: Vec<Entry>,
    token: String,
}

pub fn fetch(stream: Stream) -> Result<String, Box<dyn Error>> {
    Ok(ureq::get(stream.get_url()?)
        .call()?
        .body_mut()
        .read_to_string()?)
}

impl Flux {
    pub fn parse(&self, content: String) -> Result<_Feed, Box<dyn Error>> {
        let feed = feed_rs::parser::parse(content.as_bytes())?;
        if feed.feed_type == FeedType::Atom {
            self.parseAtom(feed)
        } else {
            Err(format!("unsupported xml format {:?}", feed.feed_type).into())
        }
    }

    fn parseAtom(feed: Feed) -> Result<_Feed, Box<dyn Error>> {}
}
