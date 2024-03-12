use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{ Pool, MySql};

use crate::models::login::{LoginRes, User};
use crate::db_access::LoginQuery;
use crate::utils::jwt::Claims;
use crate::errors::error::Myerror;

pub async fn login(
    State(pool): State<Pool<MySql>>,
    Json(user): Json<User>
) -> Result<Json<LoginRes>,Myerror>  {

    let sql_box = LoginQuery::get_logininfo(&pool, user.username()).await;

    match sql_box {
        Ok(db_user) => {
            if db_user.password() == user.password() {
                let token = encode(
                    &Header::default(), 
                    &Claims::new(format!("username:{},password:{}",db_user.username(),db_user.password())), 
                &EncodingKey::from_secret("secret".as_ref())).unwrap();
                let res = LoginRes::new(StatusCode::OK, String::from("密码正确"), Some(token));
                return Ok(Json(res))
            } else {
                let res = LoginRes::new(StatusCode::OK, String::from("密码错误"), None);
                return Ok(Json(res))
            };
        },
        Err(sqlx::Error::RowNotFound) => {
            let res = LoginRes::new(StatusCode::OK, String::from("账户不存在"), None);

            return Ok(Json(res))
        },
        _ => {
            return Err(Myerror::DBError(String::from("数据库有其他错误")))
        }
    }
}