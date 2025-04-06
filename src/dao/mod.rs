use crate::entity::config::MySQLConfig;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use rocket::fairing::AdHoc;
use std::sync::Arc;

mod mapper;
pub mod user_info_dao;

// 定义全局变量
lazy_static! {
    // Rbatis 类型变量 RB，用于数据库查询
    pub static ref RB: RBatis = RBatis::new();
}

pub fn rbatis_stage(config: &Option<MySQLConfig>) -> AdHoc {
    let mysql_uri = match config {
        None => {
            panic!("MySQL configuration is missing");
        }
        Some(mysql_config) => {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                mysql_config.user,
                mysql_config.password,
                mysql_config.host,
                mysql_config.port,
                mysql_config.schema
            )
        }
    };

    AdHoc::on_ignite("Rbatis Drivers", |rocket| async move {
        RB.link(MysqlDriver {}, &mysql_uri).await.unwrap();
        let rb = Arc::new(&RB);

        rocket.manage(rb)
    })
}
