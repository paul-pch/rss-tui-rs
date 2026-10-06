use serde::Deserialize;
use std::error::Error;
use std::{env, fs};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stream {
    pub name: String,
    url: String,
    token_key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub streams: Vec<Stream>,
}

impl Stream {
    pub fn get_url(&self) -> Result<String, Box<dyn Error>> {
        match &self.token_key {
            Some(key) => {
                let token = env::var(key).map_err(|e| format!("Fail to get env var: {key} {e}"))?;
                Ok(format!("{}?feed_token={}", self.url, token))
            }
            None => Ok(self.url.clone()),
        }
    }
}

fn parse_config(content: &str) -> Result<Config, Box<dyn Error>> {
    Ok(toml::from_str(content).map_err(|e| format!("Fail to parse config file: {e}"))?)
}

pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path).map_err(|e| format!("Fail to load config file: {e}"))?;
    let config = parse_config(&content)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_fail_loading_missing_file() {
        let err = load("/nonexistent/config.toml").unwrap_err();
        assert!(err.to_string().contains("Fail to load"));
    }

    #[test]
    fn test_should_succeed_empty_config() {
        let toml = "";
        let config: Config = parse_config(toml).unwrap();
        assert!(config.streams.is_empty());
    }

    #[test]
    fn test_should_fail_parsing_missing_field() {
        let toml = r#"
            [[streams]]
            name = "blog"
        "#;
        let err = parse_config(toml).unwrap_err();
        assert!(err.to_string().contains("Fail to parse config file"));
        assert!(err.to_string().contains("missing field `url`"));
    }

    #[test]
    fn test_should_fail_parsing_unknown_field() {
        let toml = r#"
            titi = "tata"
        "#;
        let err = parse_config(toml).unwrap_err();
        assert!(err.to_string().contains("Fail to parse config file"));
        assert!(err.to_string().contains("unknown field `titi`"));
    }
}
