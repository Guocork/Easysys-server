use std::env;
use sqlx::{ mysql::MySqlPoolOptions, MySql, Pool};
use dotenv::dotenv;



pub async fn establish_connection() -> Pool<MySql> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("没有设置数据库地址");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("没有连接到数据库");

    pool
}