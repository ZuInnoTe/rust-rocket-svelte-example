#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/public", FileServer::from("./static"))
}

#[cfg(test)]
mod tests {

    #[rocket::async_test]
    // Test the index
    async fn test_index() {
        let result = super::index().await;
        let expected = "Hello, world!".to_string();
        assert_eq!(expected, result);
    }
}
