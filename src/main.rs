mod config;
mod flux;

use config::Config;
use std::error::Error;

use crate::{config::load, flux::Feed};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let config: Config = load("config.toml")?;

    if config.streams.is_empty() {
        println!("No stream configured in config file.");
    }

    let mut feeds: Vec<Feed> = Vec::new();

    for stream in &config.streams {
        let xml = flux::fetch(stream)?;
        feeds.push(flux::parse(&xml, stream.name.clone())?);
    }

    Ok(())
}
