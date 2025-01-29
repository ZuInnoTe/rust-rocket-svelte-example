use rocket::fairing::{self};
use rocket::{Build, Rocket};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("warehouse")]
pub struct Db(sqlx::SqlitePool);

pub type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

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
