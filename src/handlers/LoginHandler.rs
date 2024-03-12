use axum::extract::State;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use sqlx::{ Pool, MySql};


use crate::models::responsedata::ResBox;
use crate::db_access::LoginQuery;
use crate::utils::jwt::Claims;
use crate::errors::error::Myerror;


#[derive(Deserialize, sqlx::FromRow)]
pub struct User {
    username: String,
    password: String
}

impl User {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}


pub async fn login<T>(
    State(pool): State<Pool<MySql>>,
    Json(user): Json<User>
) -> Result<Json<ResBox<T>>,Myerror> {

    let sql_box = LoginQuery::get_logininfo(&pool, user.username()).await;

    match sql_box {
        Ok(db_user) => {
            if db_user.password() == user.password() {

                let token = encode(
                    &Header::default(), 
                    &Claims::new(format!("username:{},password:{}",db_user.username(),db_user.password())), 
                &EncodingKey::from_secret("secret".as_ref())).unwrap();

                let res = ResBox::new(
                    String::from("OK"), 
                    String::from("密码正确"), 
                    None);

                return Ok(Json(res))
            } else {
                let res = ResBox::new(
                    String::from("OK"),
                    String::from("密码错误"),
                    None);

                return Ok(Json(res))
            };
        },
        Err(sqlx::Error::RowNotFound) => {
            let res = ResBox::new(
                String::from("OK"), 
                String::from("账户不存在"), 
                None);

            return Ok(Json(res))
        },
        _ => {
            return Err(Myerror::DBError(String::from("数据库有其他错误")))
        }
    }
}