//! Rocket handler to redirect routes of the SPA frontend to the SPA frontend and not handling them in the backend

use crate::oidc::guard::OidcUser;

use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;

/// Handler to redirect routes of the SPA front end to the SPA frontend
///
/// # Arguments
/// * `path` - path of front-end routes to be routed to the SPA frontend
///
#[get("/<path..>")]
pub async fn spa_redirect_frontend_route(path: PathBuf, user: OidcUser) -> Option<NamedFile> {
    let path = Path::new("./static").join("index.html");
    NamedFile::open(path).await.ok()
}
