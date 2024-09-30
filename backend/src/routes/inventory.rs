
#[get("/inventory")]
pub async  fn inventory() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    // Test the index
    async fn test_hello() {
        let result = super::inventory().await;
        let expected = "Hello, world!".to_string();
        assert_eq!(expected, result);
    }
}