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
) -> Json<LoginRes>  {

    let db_user = LoginQuery::get_logininfo(&pool, user.username()).await;

    if db_user.password() == user.password() {
        let token = encode(
            &Header::default(), 
            &Claims::new(format!("username:{},password:{}",db_user.username(),db_user.password())), 
        &EncodingKey::from_secret("secret".as_ref())).unwrap();
        let res = LoginRes::new(StatusCode::OK, String::from("密码正确"), Some(token));
        return Json(res)
    } else {
        let res = LoginRes::new(StatusCode::BAD_REQUEST, String::from("密码错误"), None);
        return Json(res)
    };
}