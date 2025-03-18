//! Rocket fairing to inject HTTP security headers, such as content security policies, into responses

use std::io;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rand::prelude::*;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::{Data, Request, Response};
use tracing::{Level, event};

// Configuration of Fairing
#[derive(Clone)]
pub struct SecurityHttpHeaders {
    pub config: crate::configuration::config::CustomAppConfig,
    pub regex_paths: regex::RegexSet,
}

// Fairing implementation
#[rocket::async_trait]
impl Fairing for SecurityHttpHeaders {
    fn info(&self) -> Info {
        Info {
            name: "HTTPFirewall - Security HTTP Headers",
            kind: Kind::Request | Kind::Response | Kind::Singleton,
        }
    }

    /// Executed on every request (we do not need it, but it is part of the Fairing trait)
    ///
    /// # Arguments
    /// * `self` - Struct Security HTTP Headers
    /// * `req` - Request object
    ///
    ///
    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        // do nothing
    }

    /// Executed on every response. We inject here the HTTP Security Headers
    ///
    /// # Arguments
    /// * `self` - Struct Security HTTP Headers
    /// * `req` - Request object
    /// * `res` - Response object
    ///
    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if (res.status() == Status::Ok) {
            // Configure Content-Security Policy Header and insert nonces
            let mut body_bytes = res.body_mut().to_bytes().await.unwrap();
            match self.config.clone().httpheaders.content_security_policy {
                Some(csp) => {
                    // check if csp is set
                    let mut csp_value = csp;
                    // check if nonce should be inserted
                    if self.regex_paths.len() > 0 {
                        let mut rng = rand::rng();
                        let mut random_bytes = [0u8; 64];
                        rng.fill_bytes(&mut random_bytes);
                        let random_nonce = BASE64_STANDARD.encode(random_bytes);
                        // insert into tag
                        match self
                            .config
                            .clone()
                            .httpheaders
                            .content_security_policy_inject_nonce_tags
                        {
                            Some(csp_tag_list) => {
                                if self.regex_paths.is_match(req.uri().path().as_str()) {
                                    event!(Level::DEBUG, "Inserting nonce in selected tags");
                                    match self
                                        .config
                                        .clone()
                                        .httpheaders
                                        .content_security_policy_nonce_headers
                                    {
                                        Some(csp_nonce_headers) => {
                                            for csp_nonce_header in csp_nonce_headers {
                                                csp_value = csp_value.replace(
                                                    &csp_nonce_header,
                                                    format!(
                                                        "{} 'nonce-{}'",
                                                        csp_nonce_header, random_nonce
                                                    )
                                                    .as_str(),
                                                );
                                            }
                                            let mut updated_body: String =
                                                String::from_utf8_lossy(&body_bytes).into();
                                            for csp_tag in csp_tag_list {
                                                updated_body = updated_body.replace(
                                                    format!("<{}", csp_tag).as_str(),
                                                    format!(
                                                        "<{} nonce=\"{}\"",
                                                        csp_tag, random_nonce
                                                    )
                                                    .as_str(),
                                                );
                                            }
                                            body_bytes = updated_body.into_bytes();
                                        }
                                        None => event!(
                                            Level::ERROR,
                                            "Configuration: Content-Security-Policy: You did not specify for which CSP attribute a nonce should be added"
                                        ),
                                    }
                                }
                            }
                            None => event!(
                                Level::ERROR,
                                "Configuration: Content-Security-Policy: You did not specify a tag to inject a nonce"
                            ),
                        }
                        res.set_raw_header("Content-Security-Policy", csp_value);
                        res.set_sized_body(body_bytes.len(), io::Cursor::new(body_bytes));
                        // Set other HTTP Security Headers
                        match self.config.clone().httpheaders.permissions_policy {
                            Some(permissions_policy) => {
                                res.set_raw_header("Permissions-Policy", permissions_policy);
                            }
                            None => (),
                        };
                        match self.config.clone().httpheaders.referrer_policy {
                            Some(referrer_policy) => {
                                res.set_raw_header("Referrer-Policy", referrer_policy);
                            }
                            None => (),
                        };
                        match self.config.clone().httpheaders.cross_origin_embedder_policy {
                            Some(cross_origin_embedder_policy) => {
                                res.set_raw_header(
                                    "Cross-Origin-Embedder-Policy",
                                    cross_origin_embedder_policy,
                                );
                            }
                            None => (),
                        };
                        match self.config.clone().httpheaders.cross_origin_opener_policy {
                            Some(cross_origin_opener_policy) => {
                                res.set_raw_header(
                                    "Cross-Origin-Opener-Policy",
                                    cross_origin_opener_policy,
                                );
                            }
                            None => (),
                        };
                        match self.config.clone().httpheaders.cross_origin_resource_policy {
                            Some(cross_origin_resource_policy) => {
                                res.set_raw_header(
                                    "Cross-Origin-Resource-Policy",
                                    cross_origin_resource_policy,
                                );
                            }
                            None => (),
                        };
                    }
                }
                None => (),
            };
        }
    }
}
