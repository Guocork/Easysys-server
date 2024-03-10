use axum::{
    routing::{get, post},
    Router,
};


use crate::handlers::LoginHandler;
use crate::state::db::establish_connection;

pub async fn crate_router() -> Router{

    let pool = establish_connection().await;

    let app = Router::new()
        .route("/login", post(LoginHandler::login))
        .with_state(pool);

    app
}