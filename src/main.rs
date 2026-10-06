mod config;
mod flux;

use config::Config;
use std::error::Error;

use crate::config::load;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let config: Config = load("config.toml")?;

    if config.streams.is_empty() {
        println!("No stream configured in config file.");
    }


    // Bloc HTTP
    config.streams.iter().map(|s| )
    // à partir de mes urls faire un appel http et en récupérer un xml (BORD)
    // string -> url
    // url to string (xml)

    // Bloc flux

    // convertir ce xml en flux (COEUR)
    // prend un string, parse et renvoie Flux
    // afficher le flux (COEUR)

    Ok(())
}
