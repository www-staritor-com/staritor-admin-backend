use crate::entity::base::config::MySQLConfig;
use once_cell::sync::OnceCell;
use rocket::fairing::AdHoc;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use std::time::Duration;

mod mapper;
pub mod user_info_dao;
pub mod resource_dao;

static DB_POOL: OnceCell<DatabaseConnection> = OnceCell::new();

async fn init_db_pool(mysql_uri: &str) {
    let mut opt = ConnectOptions::new(mysql_uri);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to the database");
    DB_POOL
        .set(db)
        .expect("Failed to set the global database connection");
}

fn conn() -> &'static DatabaseConnection {
    DB_POOL
        .get()
        .expect("Database connection is not initialized")
}

pub fn sea_orm_stage(config: &Option<MySQLConfig>) -> AdHoc {
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

    AdHoc::on_ignite("Sea-Orm Drivers", |rocket| async move {
        init_db_pool(mysql_uri.as_str()).await;
        rocket.manage(Arc::new(conn()))
    })
}
