use axum::{Router, routing::post, response::IntoResponse, http::StatusCode, Json};
use epiphany_utls::{warp_response};

use crate::{domain::req::login::{LoginByEmailReq, LoginByPhoneReq}, service::login as login_service};

pub fn init_router() -> Router {
    auth_router().merge(un_auth_router())
}

/**
 */
fn auth_router() -> Router {
    Router::new().route("/login_by_email", post(login_by_email))
}

fn un_auth_router() -> Router {
    Router::new().route("/login_by_phone", post(login_by_phone))
}
/**
 * 
 */
async fn login_by_email(Json(payload): Json<LoginByEmailReq>) -> impl IntoResponse {
    let resp = login_service::login_by_email(payload).await;
    (StatusCode::OK,Json(resp))
}

/**
 * 
 */
async fn login_by_phone(Json(payload): Json<LoginByPhoneReq>) -> impl IntoResponse {
    let resp = login_service::login_by_phone(payload).await;
    (StatusCode::OK,Json(warp_response::WarpResponse::build(resp)))
}