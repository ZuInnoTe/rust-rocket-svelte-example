use crate::inventory::{self, product::Product};
use crate::order::order::Order;

use futures::{future::TryFutureExt, stream::TryStreamExt};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rust_decimal::prelude::*;
use time::macros::format_description;
use time::OffsetDateTime;
use tracing::{event, Level};
use uuid::Uuid;

use crate::services::sanitization;

#[get("/order")]
pub async fn order_handler(
    mut db: Connection<crate::database::Db>,
) -> crate::database::Result<Json<Vec<Order>>> {
    event!(Level::DEBUG, "order handler called");
    let format = format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour \
             sign:mandatory]:[offset_minute]"
    );
    let orders = sqlx::query!("SELECT 'order'.id as order_id,'order'.order_datetime as order_order_datetime,product.id as product_id,product.name as product_name,product.price as product_price FROM 'order' INNER JOIN product on 'order'.product_id = product.id")
    .fetch(&mut **db)
    .map_ok(|record| Order { id: Uuid::parse_str(record.order_id.unwrap().as_str()).unwrap(), order_datetime: OffsetDateTime::parse(record.order_order_datetime.unwrap().as_str(),format).unwrap(), product: Product { id: Uuid::parse_str(record.product_id.unwrap().as_str()).unwrap(),  name: record.product_name, price: Decimal::from_str(record.product_price.unwrap().as_str()).unwrap()}})
    .try_collect::<Vec<_>>()
    .await?;
    Ok(Json(orders))
}

#[cfg(test)]
mod tests {}
