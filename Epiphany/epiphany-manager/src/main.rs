use axum::{
    http::{header, Method},
    Extension, Router, Server,
};
use epiphany_manager::{handler, init_context};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer, AllowOrigin},
    trace::TraceLayer,
};

#[derive(Clone)]
struct State {}

#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    let app = Router::new()
        .nest("/manager/adminer", handler::adminer_handler::init_router())
        .nest("/manager/login", handler::login_handler::init_router())
        .nest("/manager/orgnize", handler::orgnize_handler::init_router())
        .nest(
            "/manager/category",
            handler::category_handler::init_router(),
        )
        .nest("/manager/menu", handler::admin_menu_handler::init_router())
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_headers(vec![
                    header::ACCEPT,
                    header::ACCEPT_LANGUAGE,
                    header::AUTHORIZATION,
                    header::CONTENT_LANGUAGE,
                    header::CONTENT_TYPE,
                ])
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(AllowOrigin::exact(
                    "http://localhost:3713".parse().unwrap(), // Make sure this matches your frontend url
                )),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(State {})),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
