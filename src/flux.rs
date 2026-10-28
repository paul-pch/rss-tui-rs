use feed_rs::model::{self as rs};

use crate::config::Stream;
use chrono::{DateTime, Utc};
use std::error::Error;

#[allow(dead_code)] // TODO: affichés plus tard (TUI)
#[derive(Debug)]
pub struct Entry {
    pub title: Option<String>,
    authors: Vec<String>,
    updated: Option<DateTime<Utc>>,
    links: Vec<String>,
}

#[allow(dead_code)] // TODO: affichés plus tard (TUI)
#[derive(Debug)]
pub struct Feed {
    pub title: Option<String>,
    updated: Option<DateTime<Utc>>,
    pub entries: Vec<Entry>,
}

pub fn fetch(stream: &Stream) -> Result<String, Box<dyn Error>> {
    Ok(ureq::get(stream.get_url()?)
        .call()
        .map_err(|e| format!("Failed to fetch stream {}: {e}", stream.name))?
        .body_mut()
        .read_to_string()?)
}

pub fn parse(xml: &str, stream_name: &str) -> Result<Feed, Box<dyn Error>> {
    Ok(Feed::from(feed_rs::parser::parse(xml.as_bytes()).map_err(
        |e| format!("Failed to parse content from {stream_name} : {e}"),
    )?))
}

impl From<rs::Feed> for Feed {
    fn from(feed: rs::Feed) -> Feed {
        Feed {
            title: feed.title.map(|t| t.content),
            updated: feed.updated,
            entries: feed.entries.into_iter().map(Entry::from).collect(),
        }
    }
}

impl From<rs::Entry> for Entry {
    fn from(entry: rs::Entry) -> Self {
        Entry {
            title: entry.title.map(|t| t.content),
            authors: entry.authors.into_iter().filter_map(|a| a.name).collect(),
            updated: entry.updated,
            links: entry.links.into_iter().map(|a| a.href).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_parse_atom_success() {
        let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
        <id>urn:test:feed</id>
        <title>Mon flux de test</title>
        <updated>2026-01-02T10:00:00Z</updated>

        <entry>
            <id>urn:test:entry:1</id>
            <title>Première entrée</title>
            <updated>2026-01-01T09:00:00Z</updated>
            <author><name>Ana</name></author>
            <link href="https://example.com/1"/>
        </entry>

        <entry>
            <id>urn:test:entry:2</id>
            <title>Deuxième entrée</title>
            <updated>2026-01-02T10:00:00Z</updated>
            <author><name>Bob</name></author>
            <link href="https://example.com/2"/>
        </entry>
        </feed>
        "#;
        let feed: Feed = parse(xml, "example.com").unwrap();
        assert_eq!(feed.title, Some("Mon flux de test".to_string()));
        assert_eq!(feed.entries[1].title, Some("Deuxième entrée".to_string()));
    }

    #[test]
    fn test_should_parse_atom_fail() {
        let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <Bad XML
        "#;
        let err = parse(xml, "test").unwrap_err();
        assert!(
            err.to_string()
                .contains("Failed to parse content from test")
        );
    }

    #[test]
    fn test_should_parse_atom_success_none() {
        let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
        <id>urn:test:feed</id>
        <updated>2026-01-02T10:00:00Z</updated>
        <entry>
            <id>urn:test:entry:1</id>
            <title>Première entrée</title>
            <updated>2026-01-01T09:00:00Z</updated>
            <author><name>Ana</name></author>
        </entry>
        </feed>
        "#;
        let feed: Feed = parse(xml, "example.com").unwrap();
        assert_eq!(feed.title, None);
        assert_eq!(feed.entries[0].title, Some("Première entrée".to_string()));
        assert!(feed.entries[0].links.is_empty());
    }
}
