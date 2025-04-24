#[macro_use]
extern crate rocket;

use crate::entity::base::config::Config;
use fast_log::plugin::file_split::{DateType, KeepType, Rolling, RollingType};
use fast_log::plugin::packer::LogPacker;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

mod controller;
mod dao;
mod entity;
mod util;

#[launch]
fn rocket() -> _ {
    if !cfg!(debug_assertions) {
        fast_log::init(
            fast_log::Config::new()
                .console()
                .chan_len(Some(100000))
                .file_split(
                    "logs/",
                    Rolling::new(RollingType::ByDate(DateType::Day)),
                    KeepType::KeepNum(2),
                    LogPacker {},
                ),
        )
        .unwrap();
    }

    let rocket = rocket::build();
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("config");

    rocket
        .attach(get_cors())
        .attach(controller::stage("/".to_owned()))
        .attach(dao::sea_orm_stage(&config.mysql))
}

pub fn get_cors() -> Cors {
    // 允许访问的域，这里允许全部，如果要指定其他可以
    // let allowed_origins = AllowedOrigins::some_exact(&["https://www.acme.com"]);
    let allowed_origins = AllowedOrigins::All;
    // You can also deserialize this
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        // 指定header：AllowedHeaders::some(&["Authorization", "Accept"]),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("cors config error")
}
