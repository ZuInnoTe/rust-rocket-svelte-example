use crate::order::order::order;


#[get("/order")]
pub async fn order_handler() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    // Test the index
    async fn test_hello() {
        let result = super::order_handler().await;
        let expected = "Hello, world!".to_string();
        assert_eq!(expected, result);
    }
}