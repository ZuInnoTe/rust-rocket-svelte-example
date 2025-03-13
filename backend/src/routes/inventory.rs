//! Rocket handler to manage inventory

use crate::inventory::{self, product::Product};
use futures::stream::TryStreamExt;
use rocket::serde::json::Json;

use rocket_db_pools::Connection;
use rust_decimal::prelude::*;
use tracing::{Level, event};
use uuid::Uuid;

/// Hnadler to list all inventory
///
/// # Arguments
/// * `db` - Async connection object to the database
///
#[get("/inventory")]
pub async fn inventory_handler(
    mut db: Connection<crate::database::Db>,
) -> crate::database::Result<Json<Vec<inventory::product::Product>>> {
    event!(Level::DEBUG, "inventory handler called");
    let products = sqlx::query!("SELECT id,name,price FROM product")
        .fetch(&mut **db)
        .map_ok(|product| Product {
            id: Uuid::parse_str(product.id.unwrap().as_str()).unwrap(),
            name: product.name,
            price: Decimal::from_str(product.price.unwrap().as_str()).unwrap(),
        })
        .try_collect::<Vec<_>>()
        .await?;
    Ok(Json(products))
}

#[cfg(test)]
mod tests {}
