use rocket::serde::{Deserialize, Serialize};
use time::macros::{date, datetime};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::inventory::product::Product;
use rust_decimal_macros::dec;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Order {
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub order_datetime: OffsetDateTime,
    pub product: Product,
}
