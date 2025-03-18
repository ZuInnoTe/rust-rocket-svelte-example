//! Rocket route to manage orders

use crate::inventory::{self, product::Product};
use crate::oidc::guard::OidcUser;
use crate::order::order::Order;
use crate::services::sanitization;

use futures::stream::TryStreamExt;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rust_decimal::prelude::*;
use time::OffsetDateTime;
use time::macros::format_description;
use tracing::{Level, event};
use uuid::Uuid;

/// Handler to list all orders
///
/// # Arguments
/// * `db` - Async connection object to the database
/// * `user` - Authenticated user (no access for unauthenticated users)
///
/// # Returns
/// All orders from the database
///
#[get("/order")]
pub async fn order_handler(
    mut db: Connection<crate::database::Db>,
    user: OidcUser,
) -> crate::database::Result<Json<Vec<Order>>> {
    event!(Level::DEBUG, "order handler called");
    let format = format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour \
             sign:mandatory]:[offset_minute]"
    );
    let orders = sqlx::query!("SELECT 'order'.id as order_id,'order'.order_datetime as order_order_datetime,product.id as product_id,product.name as product_name,product.price as product_price FROM 'order' INNER JOIN product on 'order'.product_id = product.id")
    .fetch(&mut **db)
    .map_ok(|record| Order { id: Uuid::parse_str(record.order_id.unwrap().as_str()).unwrap(), order_datetime: OffsetDateTime::parse(record.order_order_datetime.unwrap().as_str(),format).unwrap(), product: Product { id: Uuid::parse_str(record.product_id.unwrap().as_str()).unwrap(),  name: sanitization::clean_all_html(record.product_name.as_str()), price: Decimal::from_str(record.product_price.unwrap().as_str()).unwrap()}})
    .try_collect::<Vec<_>>()
    .await?;
    Ok(Json(orders))
}

#[cfg(test)]
mod tests {}
