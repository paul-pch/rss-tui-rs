use feed_rs::model::{self as rs};

use crate::config::Stream;
use chrono::{DateTime, Utc};
use std::error::Error;

struct Entry {
    title: Option<String>,
    author: Vec<String>,
    updated: Option<DateTime<Utc>>,
    link: Vec<String>,
}

pub struct Feed {
    title: String,
    updated: Option<DateTime<Utc>>,
    entries: Vec<Entry>,
}

pub fn fetch(stream: &Stream) -> Result<String, Box<dyn Error>> {
    Ok(ureq::get(stream.get_url()?)
        .call()?
        .body_mut()
        .read_to_string()?)
}

pub fn parse(xml: &String, stream_name: String) -> Result<Feed, Box<dyn Error>> {
    Ok(Feed::from(feed_rs::parser::parse(xml.as_bytes()).map_err(
        |e| format!("Fail to parse content from {stream_name} :  {e}"),
    )?))
}

impl From<rs::Feed> for Feed {
    fn from(feed_atom: rs::Feed) -> Feed {
        Feed {
            title: feed_atom.title.unwrap().content,
            updated: feed_atom.updated,
            entries: feed_atom
                .entries
                .into_iter()
                .map(|e| Entry::from(e))
                .collect(),
        }
    }
}

impl From<rs::Entry> for Entry {
    fn from(entry_atom: rs::Entry) -> Self {
        Entry {
            title: entry_atom.title.unwrap().src,
            author: entry_atom
                .authors
                .into_iter()
                .map(|a| a.name.unwrap())
                .collect(),
            updated: entry_atom.updated,
            link: entry_atom.links.into_iter().map(|a| a.href).collect(),
        }
    }
}
