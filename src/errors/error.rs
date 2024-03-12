use axum::response::IntoResponse;
use axum::http::StatusCode;


pub enum Myerror {
    DBError(String),
    AxumError(String),
    NetError(String)
}

impl IntoResponse for Myerror {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

