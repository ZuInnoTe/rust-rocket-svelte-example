//! Example for a web application based on Rocket for the backend and SvelteKit for the FrontEnds

#[macro_use]
extern crate rocket;

use configuration::config::configure_fileserver;
use configuration::config::configure_oidc;

use configuration::config::read_security_http_headers_config;

use rocket::fairing::{self, AdHoc};
use rocket::fs::{FileServer, Options, relative};

use rocket::shield::Shield;

use rocket_db_pools::Database;

pub mod configuration;
pub mod database;
pub mod httpfirewall;
pub mod inventory;
pub mod oidc;
pub mod order;
pub mod routes;
pub mod services;

/// Launch endpoints of the web application
#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        // shield
        .attach(Shield::default())
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
        .mount("/", routes![crate::routes::static_serve::serve_static])
        // redirect to authentication
        .mount("/", routes![crate::oidc::routes::redirect_auth]);
    // read application config
    let figment = rocket.figment();

    let config: crate::configuration::config::Config = figment.extract().expect("config");
    // configure file server
    let rocket = configure_fileserver(rocket, &config);
    // configure fairing for http security headers
    let rocket = rocket.attach(read_security_http_headers_config(&config));
    // create oidc
    configure_oidc(rocket, &config)
}
