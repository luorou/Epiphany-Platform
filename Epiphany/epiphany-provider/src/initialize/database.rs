use crate::APP_CONTEXT;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

pub async fn init_database() {
    dotenv::dotenv().ok();
    let mysql_db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut mysql_opt = ConnectOptions::new(mysql_db_url.to_owned());
    mysql_opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let mysql_conn = Database::connect(mysql_opt).await.expect("mysql start failed");
    APP_CONTEXT.set::<DatabaseConnection>(mysql_conn);
    info!("database init success : ({})...", mysql_db_url);
}

// pub async fn get_db_connect() -> Result<DatabaseConnection,String> {

//     Ok(1)
// }
