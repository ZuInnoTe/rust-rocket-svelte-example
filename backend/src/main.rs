//! Example for a web application based on Rocket for the backend and SvelteKit for the FrontEnds

#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::fs::FileServer;
use rocket::fs::Options;
use rocket_db_pools::Database;

pub mod database;
pub mod inventory;
pub mod order;
pub mod routes;

/// Launch endpoints of the web application
#[launch]
fn rocket() -> _ {
    rocket::build()
        // database
        .attach(crate::database::Db::init())
        .attach(AdHoc::try_on_ignite(
            "Development Migrations",
            crate::database::run_migrations,
        ))
        // backend API for the frontend
        .mount(
            "/ui-api",
            routes![
                crate::routes::inventory::inventory_handler,
                crate::routes::order::order_handler
            ],
        )
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
