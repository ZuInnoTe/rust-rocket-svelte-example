use regex::Regex;
use rocket::serde::Deserialize;

use crate::httpfirewall::securityhttpheaders::SecurityHttpHeaders;

// Configuration

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CustomAppConfig {
    pub content_security_policy: Option<String>,
    pub content_security_policy_inject_nonce_paths: Option<Vec<String>>,
    pub content_security_policy_inject_nonce_tags: Option<Vec<String>>,
    pub content_security_policy_nonce_headers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub app: CustomAppConfig,
}

pub fn read_security_http_headers_config(config: Config) -> SecurityHttpHeaders {
    let mut regex_paths = regex::RegexSet::new(Vec::<String>::new()).unwrap();
    match &config.app.content_security_policy_inject_nonce_paths {
        Some(regex_path_vec) => {
            regex_paths = regex::RegexSet::new(regex_path_vec).unwrap();
        }
        None => (),
    }

    SecurityHttpHeaders {
        config: config.app,
        regex_paths: regex_paths,
    }
}
