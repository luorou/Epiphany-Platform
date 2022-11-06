use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use epiphany_utls::{ warp_response};

use crate::service;

pub fn init_router() -> Router {
    Router::new()
        // .route("/crate_category", post(crate_category))
        .route("/list_category", post(list_category))
}


async fn list_category() -> impl IntoResponse {
    let db_result = service::app_category::all().await;
    match db_result {
        Ok(v) => {
            return (StatusCode::OK,Json(warp_response::WarpResponse::build(Ok(v))));
        },
        Err(e) => {
            return (StatusCode::OK,Json(warp_response::WarpResponse::build(Err(Some(String::from(e.to_string()))))));
        },
    }
    // return (StatusCode::OK,Json(warp_response::WarpResponse::build(Ok(db_result))));
}
