//! Order processing functionality

use rocket::serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::inventory::product::Product;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Order {
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub order_datetime: OffsetDateTime,
    pub product: Product,
}
