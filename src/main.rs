#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use crate::entity::config::Config;
use fast_log::plugin::file_split::{DateType, KeepType, Rolling, RollingType};
use fast_log::plugin::packer::LogPacker;

mod controller;
mod dao;
mod entity;

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
        .attach(controller::stage("/".to_string()))
        .attach(dao::rbatis_stage(&config.mysql))
}
