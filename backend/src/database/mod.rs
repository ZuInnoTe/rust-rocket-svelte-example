//! Manage database and create/update schemas

use rocket::fairing::{self};
use rocket::{Build, Rocket};
use rocket_db_pools::Database;
use rocket_db_pools::sqlx::{self};

#[derive(Database)]
#[database("warehouse")]
pub struct Db(sqlx::SqlitePool);

pub type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

/// Create/update schemas in database
///
/// # Arguments
/// * `rocket` - rocket variable representing the Rocket instance
///
/// # Returns
/// Fairing that can be attached using rocket.attach
///
pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/sqlx/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}
