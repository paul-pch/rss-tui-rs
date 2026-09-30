mod config;

use config::Config;
use std::error::Error;

use crate::config::load;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let config: Config = load("config.toml")?;

    if config.streams.is_empty() {
        println!("No stream configured in config file.");
    }

    for stream in &config.streams {
        println!("{}", stream.name);
        println!("{}", stream.host);
        println!("{}", stream.port);
        println!("{}", stream.path);
        println!("{}", stream.token_env_key);
    }

    Ok(())
}
