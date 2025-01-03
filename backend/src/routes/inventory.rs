use crate::inventory;
use rocket::serde::json::Json;

#[get("/inventory")]
pub async fn inventory_handler() -> Json<Vec<inventory::product::Product>> {
    match inventory::inventory::get_all_products() {
        Some(product_list) => Json(product_list),
        None => Json(Vec::new())
    }
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    // Test the index
    async fn test_hello() {
        let result = super::inventory_handler().await;
        let expected = 0;
        assert_eq!(expected, result.len());
    }
}
