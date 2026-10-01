use crate::config::Stream;
use std::env;
use std::error::Error;
use toml::value::Date;

struct Entry {
    title: String,
    author: String,
    updated: Date,
    link: String,
}

struct Feed {
    url: String,
    title: String,
    updated: Date,
    entries: Vec<Entry>,
    token: String,
}

// fn parse(stream: Stream) -> Result<Feed, Box<dyn std::error::Error>> {
//     // Concat url
//     let url = concat_url(stream)?;

//     // Télécharger le fichier
//     // Créer objet Feed

//     Ok(())
// }

fn concat_url(stream: Stream) -> Result<String, Box<dyn Error>> {
    let scheme = "https://";
    let fqdn = &stream.host;
    let path = &stream.path;

    let mut base = format!("{scheme}{fqdn}");

    // Add port
    if let Some(p) = stream.port {
        base.push_str(format!(":{p}").as_str());
    }

    // Add path
    base.push_str(format!("/{path}").as_str());

    match resolve_token(&stream)? {
        Ok(token) => base.push_str(format!("?feed_token={}", token.unwrap()).as_str()),
        Err(_) => (),
    }

    Ok(base)
}

// IF a key exists, the env value should exists to build the full url
fn resolve_token(stream: &Stream) -> Result<Option<String>, Box<dyn Error>> {
    let Some(key) = &stream.token_env_key else {
        return Ok(None);
    };

    let token = env::var(key)
        .map_err(|e| format!("Stream '{}': cannot read env var {key}: {e}", stream.name))?;

    Ok(Some(token))
}

// ========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_fail_missing_key() {
        let stream = Stream {
            name: "test".to_owned(),
            host: "test.com".to_owned(),
            port: Some(8080),
            path: "path".to_owned(),
            token_env_key: Some(String::from("secret")),
        };

        let err = concat_url(stream).unwrap_err();
        assert!(err.to_string().contains("secret missing"));
    }

    #[test]
    fn test_should_concat_with_port() {
        let stream = Stream {
            name: "test".to_owned(),
            host: "test.com".to_owned(),
            port: Some(8080),
            path: "path".to_owned(),
            token_env_key: Some(String::from("secret")),
        };

        let full = concat_url(stream).unwrap();
        assert_eq!("https://test.com:8080/path?feed_token=secret", full);
    }

    #[test]
    fn test_should_concat_without_port() {
        let stream = Stream {
            name: "test".to_owned(),
            host: "test.com".to_owned(),
            port: None,
            path: "path".to_owned(),
            token_env_key: Some(String::from("secret")),
        };

        let full = concat_url(stream).unwrap();
        assert_eq!("https://test.com/path?feed_token=secret", full);
    }
}
