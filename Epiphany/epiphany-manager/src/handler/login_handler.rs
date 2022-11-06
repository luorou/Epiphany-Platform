use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use epiphany_utls::{ warp_response, jwt_utils};
use crate::domain::req::login_req::LoginReq;
use crate::service::login_service;

pub fn init_router() -> Router {
    auth_router().merge(un_auth_router())
}

/**
 */
fn auth_router() -> Router {
    Router::new().route("/login_by_account", post(login_by_account))
}

fn un_auth_router() -> Router {
    Router::new().route("/login_by_phone", post(login_by_phone))
}
/**
 * 
 */
async fn login_by_account(Json(payload): Json<LoginReq>) -> impl IntoResponse {
    let resp = login_service::login_by_account(payload).await;
    (StatusCode::OK,Json(warp_response::WarpResponse::build(resp)))
}

/**
 * 
 */
async fn login_by_phone(Json(payload): Json<LoginReq>) -> impl IntoResponse {
   let res=  jwt_utils::decode_token(payload.account);
    match res {
        Ok(t) => (
            StatusCode::OK,
            Json(warp_response::WarpResponse::build(Ok(t)))
        ),
        Err(e) => (
            StatusCode::OK,
            Json(warp_response::WarpResponse::build(Err(e)))
        ),
    }
}