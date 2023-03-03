use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub mod errors;

pub type Db = Pool<MySql>;

pub struct Database(pub Db);

pub struct JwtKey(pub String);

pub async fn get_connection() -> Db {
    dotenv().ok();
    let conn_string = std::env::var("DATABASE_URL").expect("DATABSE CONNECTION NOT PROVIDED");

    return MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&conn_string)
        .await
        .unwrap();
}
