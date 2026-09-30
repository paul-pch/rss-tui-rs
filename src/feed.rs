use crate::config::Stream;
use toml::value::Date;
use url::Url;

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
//     let mut url = "http://".to_owned().push_str(&stream.host);

//     match stream.port {
//         Some(port) => {
//             url = url + "/";
//         }
//         _ => (),
//     }

//     Ok(())
//     // Télécharger le fichier
//     // Créer objet Feed
// }

fn concat_url(stream: Stream) -> String {
    let scheme = "https://";
    let fqdn = stream.host;
    let path = stream.path;
    
    let secret = 
    
    let base = format!("{scheme}{fqdn}");

    if let Some(p) = stream.port {
        base.push_str(format!(":{}", p.to_string()).as_str());
    }

    if let Some(s) = stream.token_env_key {
        base.push_str(format!(":{}", p.to_string()).as_str());
    }

    format!("{base}/{path}.feed_token={}")
    url.set_path(path.as_str());

    url.to_string()
}


// ========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_concat_with_port() {
        let stream = Stream {
            name: "test".to_owned(),
            host: "test.com".to_owned(),
            port: Some(8080),
            path: "path".to_owned(),
            token_env_key: Some(String::from("secret")),
        };

        let full = concat_url(stream);
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

        let full = concat_url(stream);
        assert_eq!("https://test.com/path?feed_token=secret", full);
    }
}
