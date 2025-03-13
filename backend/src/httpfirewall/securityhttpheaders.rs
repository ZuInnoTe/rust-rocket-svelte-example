//! Rocket fairing to configure secure HTTP headers, such as content security policies

use std::io;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rand::prelude::*;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::{Data, Request, Response};
use tracing::{Level, event};

#[derive(Clone)]
pub struct SecurityHttpHeaders {
    pub config: crate::configuration::config::CustomAppConfig,
    pub regex_paths: regex::RegexSet,
}

#[rocket::async_trait]
impl Fairing for SecurityHttpHeaders {
    fn info(&self) -> Info {
        Info {
            name: "HTTPFirewall - Security HTTP Headers",
            kind: Kind::Request | Kind::Response | Kind::Singleton,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        // do nothing
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if (res.status() == Status::Ok) {
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
                    }
                }
                None => (),
            };
        }
    }
}
