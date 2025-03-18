//! Rocket handler to serve static content as FileServer in Rocket does not support authentication

use crate::{configuration::config::CustomAppStaticFilesConfig, oidc::guard::OidcUser};

use std::path::{Path, PathBuf};

use rocket::{State, fs::NamedFile};

/// Handler to serve static content
///
/// # Arguments
/// * `path` - path of front-end routes to be routed to the SPA frontend. Note: Rocket automatically makes sure that they do not allow path-traversal
/// * `fileserver_config` - configuration of the static file server
/// * `user` - provided by Rocket only if user is authenticated => otherwise handler cannot be accessed
///
/// Returns the content of the file
///
#[get("/<path..>", rank = 2)]
pub async fn serve_static(
    path: PathBuf,
    fileserver_config: &State<CustomAppStaticFilesConfig>,
    user: OidcUser,
) -> Option<NamedFile> {
    let mut path = Path::new(&fileserver_config.location).join(path);
    if path.is_dir() {
        path = path.join("index.html");
    }
    NamedFile::open(path).await.ok()
}
