//! Request guard to ensure OIDC authentication to routes in Rocket

use super::oidcflow::{OidcFlow, OidcSessionCookie};

use openidconnect::{ClaimsVerificationError, EndUserUsername, SubjectIdentifier};
use rocket::serde::json::serde_json;
use rocket::{
    State,
    http::{Cookie, Status},
    request::{self, FromRequest, Outcome, Request},
};
use serde::Serialize;
use tracing::{Level, event};

// Represents an authenticated user in a Rocket route
#[derive(Serialize)]
pub struct OidcUser {
    pub subject: SubjectIdentifier,
    pub preferred_username: Option<EndUserUsername>,
    pub mapped_roles: Vec<String>,
}

// Loads user authentication information from the oidc session cookie
impl OidcUser {
    fn load_from_session(
        oidc: &OidcFlow,
        oidc_session: &OidcSessionCookie,
    ) -> Result<OidcUser, ClaimsVerificationError> {
        let id_token_verifier = oidc.client.id_token_verifier();
        let id_token_claims = match oidc_session
            .id_token
            .claims(&id_token_verifier, &oidc.nonce)
        {
            Ok(id_token_claims) => id_token_claims,
            Err(err) => {
                return Err(err);
            }
        };

        let preferred_username = id_token_claims.preferred_username().cloned();
        let subject = id_token_claims.subject().clone();
        let mapped_roles: Vec<String> = oidc_session.mapped_roles.clone();

        Ok(OidcUser {
            subject,
            preferred_username,
            mapped_roles,
        })
    }
}

// Implementation of the request guard to ensure that the user is authenticated via OIDC

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OidcUser {
    type Error = ();

    /// Executed for each request on which route the OidcUser is included
    /// Reads from the cookie the user information including the OIDC token
    /// If they are not presented then a cookie is added from which route the user came from so the user is redirected there again after authentication
    ///
    /// # Arguments
    /// * `req` - Request object
    ///
    ///
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let mut cookies = req.cookies();
        if let Some(serialized_session) = cookies.get_private("oidc_user_session") {
            if let Ok(oidc_session) =
                serde_json::from_str::<OidcSessionCookie>(serialized_session.value())
            {
                let oidc = req.guard::<&State<OidcFlow>>().await.unwrap();

                match OidcUser::load_from_session(&oidc, &oidc_session) {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => {
                        cookies.remove_private(serialized_session);
                        Outcome::Error((Status::UnprocessableEntity, ()))
                    }
                }
            } else {
                cookies.remove_private(serialized_session);
                cookies.add_private(Cookie::new(
                    "oidc_redirect_destination",
                    req.uri().to_string(),
                ));
                Outcome::Forward(Status::Ok)
            }
        } else {
            cookies.add_private(Cookie::new(
                "oidc_redirect_destination",
                req.uri().to_string(),
            ));
            Outcome::Forward(Status::Ok)
        }
    }
}
