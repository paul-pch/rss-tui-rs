use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Stream {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub token_env_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub streams: Vec<Stream>,
}

fn parse_config(content: &str) -> Result<Config, Box<dyn std::error::Error>> {
    Ok(toml::from_str(content).map_err(|e| format!("Fail to parse config file: {e}"))?)
}

pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path).map_err(|e| format!("Fail to load config file: {e}"))?;
    let config = parse_config(&content)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing_file() {
        let err = load("/nonexistent/config.toml").unwrap_err();
        assert!(err.to_string().contains("Fail to load"));
    }

    #[test]
    fn test_should_succeed_empty_field() {
        let toml = "";
        let config: Config = parse_config(toml).unwrap();
        assert!(config.streams.is_empty());
    }

    #[test]
    fn test_should_fail_parsing_missing_field() {
        let toml = r#"
            [[streams]]
            name = "blog"
            host = "example.com"
        "#;
        let err = parse_config(toml).unwrap_err();
        assert!(err.to_string().contains("Fail to parse config file"));
        assert!(err.to_string().contains("missing field `port`"));
    }

    #[test]
    fn test_should_fail_parsing_unknown_field() {
        let toml = r#"
            titi = "tata"
        "#;
        let err = parse_config(toml).unwrap_err();
        println!("{err}");
        assert!(err.to_string().contains("Fail to parse config file"));
        assert!(err.to_string().contains("unknown field `titi`"));
    }
}
