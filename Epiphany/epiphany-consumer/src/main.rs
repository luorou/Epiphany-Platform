use axum::{Router, Server};
use epiphany_consumer::{handler, init_context};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    let app = Router::new()
        .nest("/consumer/register", handler::register_handler::init_router())
        .nest("/consumer/member", handler::member_handler::init_router())
        .nest("/consumer/login", handler::login_handler::init_router())
        .nest("/consumer/category", handler::app_category_handler::init_router());
        
    // .layer(cors);
    //     .fallback(fallback.into_service());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
