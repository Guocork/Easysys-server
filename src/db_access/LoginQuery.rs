use crate::models::login::User;
use sqlx::{ MySql, Pool};


pub async fn get_logininfo(pool: &Pool<MySql> ,username: &String) -> User  {
    let user = sqlx::query_as::<_,User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await
        .expect("查询失败");  // 这里的错误可以集中处理

    user

}