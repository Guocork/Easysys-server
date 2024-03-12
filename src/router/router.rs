use axum::{
    routing::{get, post},
    Router,
};


use crate::handlers::LoginHandler;
use crate::state::db::establish_connection;

pub async fn crate_router() -> Router{

    let pool = establish_connection().await;

    // 登录接口路由 /login
    let login_routes = Router::new()
            .route("/login", post(LoginHandler::login::<String>)); //这里需要修改 看怎么处理可以帮助编译器进行类型推断

    // 用户接口路由 /api/user
    let user_routes = Router::new()
            .route("/user", get(|| async {"hello"}));

    // api路由总路径 /api/...
    let api_routes = Router::new().nest("/api", user_routes);

    let app = Router::new()
        .merge(login_routes).merge(api_routes)
        .with_state(pool);

    app
}