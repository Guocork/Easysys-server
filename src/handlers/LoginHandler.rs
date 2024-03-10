use axum::extract::State;
use axum::Json;
use sqlx::{ Pool, MySql};

use crate::models::login::User;
use crate::db_access::LoginQuery;

pub async fn login(
    State(pool): State<Pool<MySql>>,
    Json(user): Json<User>
)  {

    let db_user = LoginQuery::get_logininfo(&pool, &user.username).await;

    println!("{}",user.username);
    let user = match db_user {
        Ok(s) => String::from("恭喜登录成功！"),
        Err(s) => String::from("账户不存在或者账号密码有问题！"),
    };
    
}