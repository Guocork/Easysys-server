use serde::Serialize;


//返回的结构体
#[derive(Serialize)]
pub struct ResBox<T> {
    status: String,
    message: String,
    data: Option<T>,
}

//构造器方法
impl<T> ResBox<T> {
    pub fn new(status: String, message: String, data: Option<T>) -> Self {
        Self { status, message, data }
    }
}
//set方法
impl<T> ResBox<T> {
    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
    pub fn set_data(&mut self, data: Option<T>) {
        self.data = data;
    }
}
//get方法
impl<T> ResBox<T> {
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn data(&self) -> &Option<T> {
        &self.data
    }
}








