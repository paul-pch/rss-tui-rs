mod config;
mod flux;

use crate::{config::load, flux::Feed};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let config = load("config.toml")?;

    if config.streams.is_empty() {
        println!("No stream configured in config file.");
    }

    let mut feeds: Vec<Feed> = Vec::new();

    for stream in &config.streams {
        let xml = flux::fetch(stream)?;
        let feed = flux::parse(&xml, &stream.name)?;
        feeds.push(feed);
    }

    for feed in feeds {
        println!("{}", feed.title.as_deref().unwrap_or("(sans nom)"));
        for entry in feed.entries {
            println!(" -> {}", entry.title.as_deref().unwrap_or("(sans nom)"));
        }
    }

    Ok(())
}
