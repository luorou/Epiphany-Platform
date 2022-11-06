use axum::{http::{Method, header}, Extension, Router, Server};
use epiphany_provider::{handler, init_context};
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
        .nest(
            "/provider/register",
            handler::register_handler::init_router(),
        )
        .nest("/provider/login", handler::login_handler::init_router())
        .nest("/provider/member", handler::member_handler::init_router())
        .nest(
            "/provider/software",
            handler::software_info_handler::init_router(),
        )
        .nest(
            "/provider/category",
            handler::software_category::init_router(),
        )
        .nest(
            "/provider/organize",
            handler::organize_handler::init_router(),
        )
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
                    "http://localhost:3715".parse().unwrap(), // Make sure this matches your frontend url
                )),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(State {})),
        );
    // .layer(cors);
    //     .fallback(fallback.into_service());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
