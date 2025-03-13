use openidconnect::reqwest;
use openidconnect::{AuthorizationCode, OAuth2TokenResponse, TokenResponse};
use rocket::serde::json::serde_json;
use rocket::{
    State,
    http::{Cookie, CookieJar},
    response::Redirect,
};
use tracing::{Level, event};

use super::oidcflow::{OidcFlow, OidcSessionCookie, handle_error};

#[derive(FromForm)]
pub struct OidcParams {
    code: String,
    state: String,
    session_state: String,
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

#[get("/oidc_redirect?<params>")]
pub async fn oidc_redirect(
    cookies: &CookieJar<'_>,
    oidc: &State<OidcFlow>,
    params: OidcParams,
) -> Result<Redirect, OidcError> {
    let http_client = match reqwest::blocking::ClientBuilder::new()
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

    let code = AuthorizationCode::new(params.code);
    let token_response = match oidc.client.exchange_code(code) {
        Ok(code) => match code.request(&http_client) {
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

    let id_token = match token_response.id_token() {
        Some(id_token) => id_token,
        None => {
            event!(Level::ERROR, "Invalid IdToken");
            return Err(OidcError::IdTokenError("Invalid IdToken".to_string()));
        }
    };
    let id_token_verifier = oidc.client.id_token_verifier();
    let claims = match id_token.claims(&id_token_verifier, &oidc.nonce) {
        Ok(claims) => claims,
        Err(err) => {
            handle_error(&err, "Invalid claims");
            return Err(OidcError::ClaimsError("Invalid claims".to_string()));
        }
    };

    let cookie = OidcSessionCookie {
        access_token: token_response.access_token().clone(),
        id_token: id_token.clone(),
        mapped_roles: Some(Vec::new()),
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
    cookies.add_private(("oidc_user_session", serialized_session_cookie));

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


#[get("/oidc_goto_auth")]
pub async fn oidc_goto_auth(oidc: &State<OidcFlow>) -> Redirect {
    let redirect_url = oidc.auth_url.to_string();
    Redirect::to(redirect_url)
} 
