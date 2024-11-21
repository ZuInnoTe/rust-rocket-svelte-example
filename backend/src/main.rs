#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::fs::Options;

pub mod routes;
pub mod inventory; 
pub mod order;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/ui-api", routes![crate::routes::inventory::inventory_handler,crate::routes::order::order_handler])
        .mount(
            "/ui",
            routes![crate::routes::redirect_frontend::spa_redirect_frontend_route],
        )
        .mount(
            "/",
            FileServer::new("./static", Options::NormalizeDirs | Options::Index),
        )
}
