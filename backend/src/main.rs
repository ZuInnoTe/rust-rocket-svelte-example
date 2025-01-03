//! Example for a web application based on Rocket for the backend and SvelteKit for the FrontEnds

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::fs::Options;

pub mod routes;
pub mod inventory; 
pub mod order;

/// Launch endpoints of the web application
#[launch]
fn rocket() -> _ {
    rocket::build()
        // backend API for the frontend
        .mount("/ui-api", routes![crate::routes::inventory::inventory_handler,crate::routes::order::order_handler])
        // redirect frontend routes that only exist in the frontend
        .mount(
            "/ui",
            routes![crate::routes::redirect_frontend::spa_redirect_frontend_route],
        )
        // deliver frontend
        .mount(
            "/",
            FileServer::new("./static", Options::NormalizeDirs | Options::Index),
        )
}
