use crate::models::login::User;
use sqlx::{ MySql, Pool};


pub async fn get_logininfo(pool: &Pool<MySql>, username: &str) -> Result<User,sqlx::Error>  {
    let user = sqlx::query_as::<_,User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;  

    Ok(user)

}