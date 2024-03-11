use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{ Pool, MySql};

use crate::models::login::{LoginRes, User};
use crate::db_access::LoginQuery;
use crate::utils::jwt::{self, Claims};

pub async fn login(
    State(pool): State<Pool<MySql>>,
    Json(user): Json<User>
) -> Json<LoginRes> {

    let db_user = LoginQuery::get_logininfo(&pool, user.username()).await;

    
    let msg = if db_user.password() == user.password() {
        String::from("密码正确")
    } else {
        String::from("密码错误")  //这里就要返回了
    };
    
    let sub = format!("username:{},password:{}",db_user.username(),db_user.password());

    let claims = Claims::new(sub);

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();

    let res = LoginRes::new(StatusCode::OK, msg, Some(token));

    Json(res)
    
}