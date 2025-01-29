use crate::order;
use rocket::serde::json::Json;

#[get("/order")]
pub async fn order_handler() -> Json<Vec<order::order::Order>> {
    match order::order::get_all_orders() {
        Some(order_list) => Json(order_list),
        None => Json(Vec::new()),
    }
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    // Test the index
    async fn test_order() {
        let result = super::order_handler().await;
        let expected = 1;
        assert_eq!(expected, result.len());
    }
}
