use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::{OffsetDateTime};
use time::macros::{date, datetime};

use rust_decimal_macros::dec;
use crate::inventory::product::{Product};


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Order {
    pub id: Uuid,
    pub order_datetime: OffsetDateTime,
    pub product: Product
}


pub fn get_all_orders() -> Option<Vec<Order>> {
    let product_1 = Product {
        id: Uuid::new_v4(),
        name: "dummyproduct".to_string(), 
        price: dec!(1.99)
     };
    let mut result: Vec<Order> = Vec::new();
    let order_1 = Order {
       id: Uuid::new_v4(),
       order_datetime: datetime!(2025-01-01 12:00:00 +00:00:00),
       product: product_1
    };
    result.push(order_1);
    Some(result)
   }