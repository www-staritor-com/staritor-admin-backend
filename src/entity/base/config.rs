use rocket::serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    address: String,
    port: u16,
    pub mysql: Option<MySQLConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MySQLConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) schema: String,
}
