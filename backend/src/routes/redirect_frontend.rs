use rocket::response::Redirect;

use std::path::{PathBuf, Path};

use rocket::fs::{NamedFile};


/// Redirect to the route of the frontend of the Single Page Application (SPA)
/// 
#[get("/<path..>")]
pub async fn spa_redirect_frontend_route(path: PathBuf) -> Option<NamedFile> {
    let  path = Path::new("./static").join("index.html");
    NamedFile::open(path).await.ok()
}