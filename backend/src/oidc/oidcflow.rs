//! OIDC client (flow) that performs the OIDC Authorization Code flow

use std::collections::HashMap;

use rocket::serde::json::serde_json;

use openidconnect::{
    AccessToken, AdditionalClaims, AuthenticationFlow, Client, ClientId, ClientSecret, CsrfToken,
    IdToken, IssuerUrl, Nonce, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    UserInfoClaims,
};

use openidconnect::core::{
    CoreGenderClaim, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm,
    CoreProviderMetadata, CoreResponseType,
};

use openidconnect::reqwest;
use openidconnect::url;
use serde::{Deserialize, Serialize};
use tracing::{Level, event};

// Errors returned by the client
#[derive(Debug)]
pub enum OAuth2Error {
    INVALID_ISSUER_URL,
    CLIENT_BUILD_ERROR,
    PROVIDER_METADATA_DISCOVERY,
}

// Structure to capture all claims in an IdToken or UserInfo endpoint response that are not part of the standard OIDC claims
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllOtherClaims(pub HashMap<String, serde_json::Value>);
impl AdditionalClaims for AllOtherClaims {}

pub type OidcAppUserInfoClaims = UserInfoClaims<AllOtherClaims, CoreGenderClaim>;

// OIDC client type that contains non-standard claims and represents the OIDC Authorisation Code flow
type OidcAppClient = Client<
    AllOtherClaims,
    openidconnect::core::CoreAuthDisplay,
    openidconnect::core::CoreGenderClaim,
    openidconnect::core::CoreJweContentEncryptionAlgorithm,
    openidconnect::core::CoreJsonWebKey,
    openidconnect::core::CoreAuthPrompt,
    openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>,
    openidconnect::StandardTokenResponse<
        openidconnect::IdTokenFields<
            AllOtherClaims,
            openidconnect::EmptyExtraTokenFields,
            openidconnect::core::CoreGenderClaim,
            openidconnect::core::CoreJweContentEncryptionAlgorithm,
            openidconnect::core::CoreJwsSigningAlgorithm,
        >,
        openidconnect::core::CoreTokenType,
    >,
    openidconnect::StandardTokenIntrospectionResponse<
        openidconnect::EmptyExtraTokenFields,
        openidconnect::core::CoreTokenType,
    >,
    openidconnect::core::CoreRevocableToken,
    openidconnect::StandardErrorResponse<openidconnect::RevocationErrorResponseType>,
    openidconnect::EndpointSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointNotSet,
    openidconnect::EndpointMaybeSet,
    openidconnect::EndpointMaybeSet,
>;

// Basic data used by the OIDC Client

pub struct OidcFlow {
    pub client: OidcAppClient,
    pub auth_url: url::Url,
    pub csrf_state: CsrfToken,
    pub nonce: Nonce,
    pub pkce_verifier_secret: String,
}

// OIDC Session cookie stores OIDC tokens and additional information, such as roles, in a cookie.
// These information are sensitive and MUST be stored ONLY in an encrypted cookie
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OidcSessionCookie {
    pub access_token: AccessToken,
    pub id_token: IdToken<
        AllOtherClaims,
        CoreGenderClaim,
        CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm,
    >,
    pub mapped_roles: Vec<String>,
}

/// Basic function to handle OIDC errors and log them
///
/// # Arguments
/// * `fail` - Error object
/// * `msg` - General error message
///
///
pub fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) {
    let mut err_msg = msg.to_string();
    let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
    while let Some(cause) = cur_fail {
        err_msg += &format!("\n    caused by: {}", cause);
        cur_fail = cause.source();
    }
    event!(Level::ERROR, "{}", err_msg);
}

// Implemetation of an OIDC client (flow) that represents the OIDC Authorization Code flow
impl OidcFlow {
    pub fn new(
        issuer_url: String,
        redirect_url: String,
        client_id: String,
        client_secret: String,
        scopes: Vec<String>,
    ) -> Result<OidcFlow, OAuth2Error> {
        // configure basic information
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);

        let issuer_url = match IssuerUrl::new(issuer_url.to_string()) {
            Ok(issuer_url) => issuer_url,
            Err(err) => {
                handle_error(&err, "Invalid issuer URL");
                return Err(OAuth2Error::INVALID_ISSUER_URL);
            }
        };
        // configure a HTTP client
        let http_client = match reqwest::blocking::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
        {
            Ok(client) => client,
            Err(err) => {
                handle_error(&err, "Cannot build client ");
                return Err(OAuth2Error::CLIENT_BUILD_ERROR);
            }
        };

        // configure provider metadata
        let provider_metadata = match CoreProviderMetadata::discover(&issuer_url, &http_client) {
            Ok(provider_metadata) => provider_metadata,
            Err(err) => {
                handle_error(&err, "Could discover provider metadata ");
                return Err(OAuth2Error::PROVIDER_METADATA_DISCOVERY);
            }
        };
        // Set up the config for the OIDC flow
        let client = OidcAppClient::from_provider_metadata(
            provider_metadata,
            client_id,
            Some(client_secret),
        )
        // set redirect URI to which the OIDC IdP should send the code
        .set_redirect_uri(
            RedirectUrl::new(redirect_url.to_string()).unwrap_or_else(|err| {
                handle_error(&err, "Invalid redirect URL");
                unreachable!();
            }),
        );

        // configure authorisation url of the OIDC IdP
        let mut authorize_url = client.authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );
        // configure scopes to be requested from the IdP
        for scope in scopes {
            authorize_url = authorize_url.add_scope(Scope::new(scope));
        }
        // Generate a PKCE challenge.
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_state, nonce) = authorize_url.url();
        Ok(OidcFlow {
            client,
            auth_url,
            csrf_state,
            nonce,
            pkce_verifier_secret: pkce_verifier.into_secret(),
        })
    }
}
