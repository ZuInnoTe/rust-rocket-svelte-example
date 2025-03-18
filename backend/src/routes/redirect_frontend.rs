//! Rocket handler to redirect routes of the SPA frontend (not of Rocket/backend) to the SPA frontend and not handling them in the backend

use crate::{configuration::config::CustomAppStaticFilesConfig, oidc::guard::OidcUser};

use std::path::{Path, PathBuf};

use rocket::{State, fs::NamedFile};

/// Handler to redirect routes of the SPA front end to the SPA frontend
///
/// # Arguments
/// * `path` - path of front-end routes to be routed to the SPA frontend
/// * `fileserver_config` - configuration of the static file server
/// * `user` - Authenticated user (no access for unauthenticated users)
///
/// # Returns
/// The SPA main page (index.html)
#[get("/<path..>")]
pub async fn spa_redirect_frontend_route(
    path: PathBuf,
    fileserver_config: &State<CustomAppStaticFilesConfig>,
    user: OidcUser,
) -> Option<NamedFile> {
    let path = Path::new(&fileserver_config.location).join("index.html");
    NamedFile::open(path).await.ok()
}
