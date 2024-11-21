use crate::inventory::product::product;


#[get("/inventory")]
pub async fn inventory_handler() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    // Test the index
    async fn test_hello() {
        let result = super::inventory_handler().await;
        let expected = "Hello, world!".to_string();
        assert_eq!(expected, result);
    }
}
