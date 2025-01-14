use crate::inventory;
use rocket::serde::json::Json;

use tracing::{event, Level};


#[get("/inventory")]
pub async fn inventory_handler() -> Json<Vec<inventory::product::Product>> {
    event!(Level::DEBUG, "inventory handler called");
    match inventory::inventory::get_all_products() {
        Some(product_list) => Json(product_list),
        None => Json(Vec::new())
    }
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    // Test the index
    async fn test_inventory() {
        let result = super::inventory_handler().await;
        let expected = 1;
        assert_eq!(expected, result.len());
    }
}
