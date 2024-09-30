#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::fs::Options;

pub mod routes;

use rocket::response::Redirect;


#[get("/")]
fn redirect_frontend() -> Redirect {
    Redirect::to("/ui")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/ui-api", routes![crate::routes::inventory::inventory])
        .mount("/ui", FileServer::new("./static",Options::NormalizeDirs | Options::Index))
        .mount("/",routes![redirect_frontend])
   
}

