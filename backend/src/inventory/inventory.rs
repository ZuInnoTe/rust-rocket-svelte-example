
#[get("/")]
pub async  fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    // Test the index
    async fn test_hello() {
        let result = super::hello().await;
        let expected = "Hello, world!".to_string();
        assert_eq!(expected, result);
    }
}
