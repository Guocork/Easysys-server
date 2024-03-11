mod api{
    pub mod LoginApi;
}
mod models {
    pub mod login;
}
mod handlers {
    pub mod LoginHandler;
}
mod state {
    pub mod db;
}
mod router {
    pub mod router;
}
mod db_access{
    pub mod LoginQuery;
}
mod utils {
    pub mod jwt;
}
use router::router::crate_router;

#[tokio::main]
async fn main() {

    let app = crate_router().await;
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


