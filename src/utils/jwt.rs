use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use std::time::{Duration, SystemTime, UNIX_EPOCH};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self{
        let exp = SystemTime::now() + Duration::from_secs(15 * 24 * 60 *60);
        let exp = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        Claims { sub, exp }
    }
}