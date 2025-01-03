use rocket::serde::{Serialize, Deserialize};


use uuid::Uuid;
use rust_decimal::prelude::*;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub id: Uuid,
    pub name: String, 
    pub price: Decimal
}