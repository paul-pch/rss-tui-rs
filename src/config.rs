use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stream {
    pub name: String,
    pub host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_env_key: Option<String>,
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
            host = "example.com"
        "#;
        let err = parse_config(toml).unwrap_err();
        assert!(err.to_string().contains("Fail to parse config file"));
        assert!(err.to_string().contains("missing field `path`"));
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
