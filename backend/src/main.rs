//! Example for a web application based on Rocket for the backend and SvelteKit for the FrontEnds

#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::fs::{FileServer, Options};

use rocket::shield::Shield;

use rocket_db_pools::Database;

pub mod configuration;
pub mod database;
pub mod httpfirewall;
pub mod inventory;
pub mod order;
pub mod routes;
pub mod services;


/// Launch endpoints of the web application
#[launch]
fn rocket() -> _ {
    let rocket= rocket::build()
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
        .mount(
            "/",
            FileServer::new("./static", Options::NormalizeDirs | Options::Index),
        );
        let figment = rocket.figment();

        // extract the entire config any `Deserialize` value
        let config: crate::configuration::config::Config = figment.extract().expect("config");
        match  config.app.content_security_policy {
            Some(csp) => println!("{}", csp),
            None => println!("no csp")
        }
        
        rocket
}
