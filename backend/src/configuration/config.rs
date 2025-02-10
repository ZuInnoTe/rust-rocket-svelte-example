use rocket::serde::Deserialize;

// Configuration

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CustomAppConfig {
    pub content_security_policy: Option<String>
}


#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
   pub app: CustomAppConfig
}
