//! Business data and logic for products

use rocket::serde::{Deserialize, Serialize};

use rust_decimal::prelude::*;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: Decimal,
}
