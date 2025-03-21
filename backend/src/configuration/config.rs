//! Manage application-specific configuration

use std::collections::HashMap;

use rocket::{Build, Rocket, serde::Deserialize};
use tracing::{Level, event};

use crate::oidc::routes::{oidc_goto_auth, oidc_redirect, oidc_user_info};

use crate::httpfirewall::securityhttpheaders::SecurityHttpHeaders;

use crate::oidc::{self, oidcflow::OidcFlow};
/// Configuration of oidc authentication/authorization
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CustomAppOidcConfig {
    pub issuer_url: Option<String>,
    pub redirect_url: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub roles_idtoken_claims: Option<Vec<String>>,
    pub roles_userinfoendpoint_claims: Option<Vec<String>>,
    pub claims_separator: Option<HashMap<String, String>>,
    pub scopes: Option<Vec<String>>,
}

/// Configuration of custom HttpHeaders
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CustomAppHttpHeadersConfig {
    pub content_security_policy: Option<String>,
    pub content_security_policy_inject_nonce_paths: Option<Vec<String>>,
    pub content_security_policy_inject_nonce_tags: Option<Vec<String>>,
    pub content_security_policy_nonce_headers: Option<Vec<String>>,
    pub permissions_policy: Option<String>,
    pub referrer_policy: Option<String>,
    pub cross_origin_embedder_policy: Option<String>,
    pub cross_origin_opener_policy: Option<String>,
    pub cross_origin_resource_policy: Option<String>,
}

/// Configuration of static file serving
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CustomAppStaticFilesConfig {
    pub location: String,
}

/// Configuration of specific modules of the app
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CustomAppConfig {
    pub httpheaders: CustomAppHttpHeadersConfig,
    pub oidc: CustomAppOidcConfig,
    pub fileserver: CustomAppStaticFilesConfig,
}

/// Custom app configuration serialized from a toml file
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub app: CustomAppConfig,
}

/// Reads from the Rocket app configuration http security headers and configures a fairing to add them to HTTP responses
///
/// # Arguments
/// * `config` - Configuration of the Rocket app
///
/// # Returns
/// Fairing that can be attached using rocket.attach
///
pub fn read_security_http_headers_config(config: &Config) -> SecurityHttpHeaders {
    let mut regex_paths = regex::RegexSet::new(Vec::<String>::new()).unwrap();
    match &config
        .app
        .httpheaders
        .content_security_policy_inject_nonce_paths
    {
        Some(regex_path_vec) => {
            regex_paths = regex::RegexSet::new(regex_path_vec).unwrap();
        }
        None => (),
    }

    SecurityHttpHeaders {
        config: config.app.clone(),
        regex_paths: regex_paths,
    }
}

/// Configure static file server for static files (e.g. frontend)
///
/// # Arguments
/// * `rocket` - variable representing a Rocket instance
/// * `config` - Configuration of the Rocket app
///
/// # Returns
/// rocket representing Rocket instance with static file server configured
///
pub fn configure_fileserver(rocket: Rocket<Build>, config: &Config) -> Rocket<Build> {
    rocket.manage(config.app.fileserver.clone())
}

/// Configure OIDC authentication with Rocket instance
///
/// # Arguments
/// * `rocket` - variable representing a rocket instance
/// * `config` - Configuration of the Rocket app
///
/// # Returns
/// rocket representing rocket instance with OIDC authentication configured
///
pub fn configure_oidc(rocket: Rocket<Build>, config: &Config) -> Rocket<Build> {
    let oidc_flow = read_oidc_config(&config);
    rocket
        .manage(oidc_flow)
        .manage(config.app.oidc.clone())
        .mount(
            "/oidc",
            routes![oidc_redirect, oidc_goto_auth, oidc_user_info],
        )
}

/// Reads the OIDC configuration and creates an OIDC client
///
/// # Arguments
/// * `config` - Configuration of the Rocket app
///
/// # Returns
/// OIDC client that should be managed in a state in Rocket
///
fn read_oidc_config(config: &Config) -> OidcFlow {
    let issuer_url = match &config.app.oidc.issuer_url {
        Some(issuer_url) => issuer_url,
        None => {
            panic!("Invalid issuer_url.");
        }
    };
    let redirect_url = match &config.app.oidc.redirect_url {
        Some(redirect_url) => redirect_url,
        None => {
            panic!("Invalid redirect_url.");
        }
    };
    let client_id = match &config.app.oidc.client_id {
        Some(client_id) => client_id,
        None => {
            panic!("Invalid client_id.");
        }
    };
    let client_secret = match &config.app.oidc.client_secret {
        Some(client_secret) => client_secret,
        None => {
            panic!("Invalid client_secret.");
        }
    };
    let scopes = match &config.app.oidc.scopes {
        Some(scopes) => scopes.clone(),
        None => Vec::new(),
    };
    match OidcFlow::new(
        issuer_url.to_string(),
        redirect_url.to_string(),
        client_id.to_string(),
        client_secret.to_string(),
        scopes,
    ) {
        Ok(oidc_flow) => oidc_flow,
        Err(err) => {
            event!(Level::ERROR, "Error initializing Oidc: {:?}", err);
            panic!("Error initializing Oidc: {:?}", err);
        }
    }
}
