use std::collections::HashMap;
use std::path::PathBuf;

use openidconnect::{AuthorizationCode, OAuth2TokenResponse, TokenResponse, reqwest};
use rocket::http::uri::Reference;
use rocket::http::{Cookie, SameSite, Status};
use rocket::serde::json::serde_json;
use rocket::{Request, Response, response};
use rocket::{State, http::CookieJar, response::Redirect};
use tracing::{Level, event};

use crate::configuration::config::CustomAppOidcConfig;

use super::guard::OidcUser;
use super::oidcflow::{self, OidcAppUserInfoClaims, OidcFlow, OidcSessionCookie, handle_error};

#[derive(FromForm)]
pub struct OidcParams {
    code: String,
    state: String,
    session_state: Option<String>,
}

#[derive(Responder)]
enum OidcError {
    #[response(status = 500)]
    ClientBuildError(String),
    CodeExchangeError(String),
    IdTokenError(String),
    ClaimsError(String),
    SerializeSessionCookie(String),
}

#[get("/redirect?<params..>")]
pub async fn oidc_redirect(
    cookies: &CookieJar<'_>,
    oidc: &State<OidcFlow>,
    oidc_config: &State<CustomAppOidcConfig>,
    params: OidcParams,
) -> Result<Redirect, OidcError> {
    // create http client to do openidconnect requests
    let http_client = match reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(client) => client,
        Err(err) => {
            handle_error(&err, "Cannot build client");
            return Err(OidcError::ClientBuildError(
                "Cannot build client".to_string(),
            ));
        }
    };
    // fetch token
    let code = AuthorizationCode::new(params.code);
    let token_response = match oidc.client.exchange_code(code) {
        Ok(code) => match code.request_async(&http_client).await {
            Ok(token_response) => token_response,
            Err(err) => {
                handle_error(&err, "Cannot exchange code");
                return Err(OidcError::CodeExchangeError(
                    "Cannot exchange code".to_string(),
                ));
            }
        },
        Err(err) => {
            handle_error(&err, "Cannot exchange code");
            return Err(OidcError::CodeExchangeError(
                "Cannot exchange code".to_string(),
            ));
        }
    };

    // verify IdToken
    let id_token = match token_response.id_token() {
        Some(id_token) => id_token,
        None => {
            event!(Level::ERROR, "Invalid IdToken");
            return Err(OidcError::IdTokenError("Invalid IdToken".to_string()));
        }
    };
    let id_token_verifier = oidc.client.id_token_verifier();
    // fetch claims and translate to roles
    let claims = match id_token.claims(&id_token_verifier, &oidc.nonce) {
        Ok(claims) => claims,
        Err(err) => {
            handle_error(&err, "Invalid claims");
            return Err(OidcError::ClaimsError("Invalid claims".to_string()));
        }
    };

    // map configured roles from IdTokenClaims
    let mut mapped_roles = Vec::new();
    mapped_roles.append(&mut parse_claims(
        claims.additional_claims().0.clone(),
        oidc_config.roles_idtoken_claims.as_ref().unwrap(),
        oidc_config,
    ));

    // map  roles from UserInfo endpoint claims
    match oidc
        .client
        .user_info(token_response.access_token().to_owned(), None)
    {
        Ok(user_info) => match user_info.request_async(&http_client).await {
            Ok(user_info_claims) => {
                let user_info_claims: OidcAppUserInfoClaims = user_info_claims;
                mapped_roles.append(&mut parse_claims(
                    user_info_claims.additional_claims().0.clone(),
                    oidc_config.roles_userinfoendpoint_claims.as_ref().unwrap(),
                    oidc_config,
                ));
            }
            Err(err) => {
                handle_error(&err, "Cannot execute request on UserInfo endpoint");
            }
        },
        Err(err) => {
            handle_error(&err, "Cannot use UserInfo endpoint");
        }
    }
    event!(Level::DEBUG, "Mapped roles from claims: {:?}", mapped_roles);
    // write information to cookie, which is privare (encrypted and tamperproof)
    let cookie = OidcSessionCookie {
        access_token: token_response.access_token().clone(),
        id_token: id_token.clone(),
        mapped_roles: mapped_roles,
    };

    let serialized_session_cookie = match serde_json::to_string(&cookie) {
        Ok(serialized_session_cookie) => serialized_session_cookie,
        Err(err) => {
            handle_error(&err, "Cannot serialize session cookie");
            return Err(OidcError::SerializeSessionCookie(
                "Cannot serialize session cookie".to_string(),
            ));
        }
    };

    // We need Samesite::Lax, because the cookie is set after a redirect to another web site. Setting it to strict can lead to infinite redirects or outdated sessions
    let session_cookie = Cookie::build(("oidc_user_session", serialized_session_cookie))
        .path("/")
        .secure(true)
        .same_site(SameSite::Lax);

    cookies.add_private(session_cookie);

    // redirect back to application
    let redirect_url = cookies
        .get_private("oidc_redirect_destination")
        .map(|crumb| format!("{}", crumb.value()));

    match redirect_url {
        Some(url) => {
            cookies.remove_private("oidc_redirect_destination");
            Ok(Redirect::to(url))
        }
        None => Ok(Redirect::to("/")),
    }
}

#[get("/login")]
pub async fn oidc_goto_auth(oidc: &State<OidcFlow>) -> Redirect {
    let redirect_url = oidc.auth_url.to_string();
    Redirect::to(redirect_url)
}

#[get("/userinfo")]
pub async fn oidc_user_info(user: OidcUser) -> String {
    match serde_json::to_string(&user) {
        Ok(json_string) => json_string,
        Err(_) => "Internal error".to_string(),
    }
}

#[get("/<path..>", rank = 3)]
pub async fn redirect_auth(path: PathBuf, user: Option<OidcUser>) -> Result<(), Redirect> {
    let user = user.ok_or_else(|| Redirect::to(uri!("/oidc/login")))?;
    Ok(())
}

fn parse_claims(
    claims: HashMap<String, serde_json::Value>,
    claims_to_extract: &Vec<String>,
    oidc_config: &State<CustomAppOidcConfig>,
) -> Vec<String> {
    let mut mapped_roles: Vec<String> = Vec::new();
    for claim in claims_to_extract {
        match claims.get(claim) {
            Some(claim_value) => {
                if claim_value.is_string() {
                    match claim_value.as_str() {
                        Some(claim_value_str) => match &oidc_config.claims_separator {
                            Some(separator_map) => match separator_map.get(claim) {
                                Some(separator) => {
                                    let claims_split =
                                        claim_value_str.split(separator).collect::<Vec<&str>>();
                                    mapped_roles
                                        .extend(claims_split.into_iter().map(|s| s.to_string()));
                                }
                                None => mapped_roles.push(claim_value_str.to_string()),
                            },
                            None => {
                                event!(Level::WARN, "No claims separator map in configuration");
                            }
                        },
                        None => event!(Level::WARN, "Claim value is str, but None was returned"),
                    }
                } else {
                    if claim_value.is_array() {
                        match claim_value.as_array() {
                            Some(claim_array) => {
                                for claim_array_value in claim_array {
                                    if (claim_array_value.is_string()) {
                                        match claim_array_value.as_str() {
                                            Some(claim_array_value_str) => {
                                                mapped_roles.push(claim_array_value_str.to_string())
                                            }
                                            None => event!(
                                                Level::WARN,
                                                "Claim value in array is a string, but none was returned"
                                            ),
                                        }
                                    } else {
                                        event!(
                                            Level::WARN,
                                            "Claim is an array, but contains non-string elements"
                                        );
                                    }
                                }
                            }
                            None => event!(
                                Level::WARN,
                                "Claim is supposed to be an array, but none was returned"
                            ),
                        }
                    } else {
                        event!(Level::ERROR, "Unknown data type for claim");
                    }
                }
            }
            None => {
                event!(Level::WARN, "No claim found for: {}", claim);
            }
        }
    }
    return mapped_roles;
}
