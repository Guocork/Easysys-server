//此文件夹用来封装请求以及响应拦截器


use axum::response::{ IntoResponse, Response};
use serde::Serialize;

//返回的结构体
#[derive(Serialize)]
pub struct Result<T> {
    code: String,
    message: String,
    data: T,
}

//构造器方法
impl<T> Result<T> {
    pub fn new(code: String, message: String, data: T) -> Self {
        Self { code, message, data }
    }
}
//set方法
impl<T> Result<T> {
    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }
}
//get方法
impl<T> Result<T> {
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn data(&self) -> &T {
        &self.data
    }
}
