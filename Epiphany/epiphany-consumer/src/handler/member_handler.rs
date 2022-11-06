use axum::http::{HeaderMap, StatusCode};
use axum::middleware::from_extractor;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use epiphany_utls::headers_utils;
use epiphany_utls::warp_response::WarpResponse;

use crate::domain::resp::member_resp::MemberResp;
use crate::middleware::auth_member;
use crate::service;

pub fn init_router() -> Router {
    auth_router().merge(un_auth_router())
}

/**
 */
fn auth_router() -> Router {
    Router::new()
        .route("/get_self_member_info", post(get_self_member_info))
        .layer(from_extractor::<auth_member::AuthMember>())
}

fn un_auth_router() -> Router {
    Router::new().route("/get_member_info", post(get_member_info))
}

/**
 * controller  get all adminer  
*/
async fn get_self_member_info(headers: HeaderMap) -> impl IntoResponse {
    let hid_result = headers_utils::get_header_id(headers, "header_id");
    
    if hid_result.is_err() {
        let err_str = hid_result.err().unwrap();
        let err_result:Result<MemberResp,Option<String>> = Result::Err(err_str);
        return (StatusCode::OK, Json(WarpResponse::build(err_result)));
    }
    
    let resp = service::member::get_member_by_id(hid_result.unwrap()).await;
    (StatusCode::OK, Json(WarpResponse::build(resp)))
}
/**
 * create adminer
 */
async fn get_member_info() -> impl IntoResponse {
    ()
}
