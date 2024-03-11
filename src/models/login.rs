use axum::http::StatusCode;
use serde::{Deserialize, Serialize};


//用于处理request
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

//用于处理response
#[derive(Serialize)]
pub struct LoginRes {
    status: u16,
    message: String,
    token: Option<String>,
}

impl LoginRes {
    pub fn new(status: StatusCode, message: String, token: Option<String>) -> Self {
        LoginRes {
            status: status.as_u16(), 
            message,
            token,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}





