use serde::{Deserialize, Serialize};

//用于处理request
#[derive(Deserialize, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub password: String
}

//用于处理response
#[derive(Serialize)]
pub struct Simresponse {
    pub msg: String
}





