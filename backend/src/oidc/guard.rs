use super::oidcflow::{OidcFlow, OidcSessionCookie};
use openidconnect::core;
use openidconnect::{Audience, ClaimsVerificationError, EndUserUsername, SubjectIdentifier};
use rocket::serde::json::serde_json;
use rocket::{
    State,
    http::{Cookie, Status},
    request::{self, FromRequest, Outcome, Request},
};

pub struct OidcUser {
    pub subject: SubjectIdentifier,
    pub audiences: Vec<Audience>,
    pub preferred_username: Option<EndUserUsername>,
    pub mapped_roles: Option<Vec<String>>,
}

impl OidcUser {
    fn load_from_session(
        oidc: &OidcFlow,
        oidc_session: &OidcSessionCookie,
    ) -> Result<OidcUser, ClaimsVerificationError> {
        let id_token_verifier: core::CoreIdTokenVerifier = oidc.client.id_token_verifier();
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
        let audiences = id_token_claims.audiences().clone();
        let mapped_roles: Option<Vec<String>> = oidc_session.mapped_roles.clone();

        Ok(OidcUser {
            subject,
            audiences,
            preferred_username,
            mapped_roles,
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OidcUser {
    type Error = ();

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
