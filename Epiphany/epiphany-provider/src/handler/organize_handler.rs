use axum::{
    http::{HeaderMap, StatusCode},
    middleware::from_extractor,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use epiphany_utls::{headers_utils, warp_response::WarpResponse};
use epiphany_entity::{tab_organize_info};

use crate::{service::oranize_service, domain::req::organize_req::CreateOrganizeReq};
use crate::{ middleware::auth_member};

pub fn init_router() -> Router {
    Router::new()
        .route("/get_orgnize_list", post(get_organize_info))
        .route("/create_organize", post(create_organize))
        .layer(from_extractor::<auth_member::AuthMember>())
}

/**
 * register_by_phone
 */
async fn get_organize_info(headers: HeaderMap) -> impl IntoResponse {
    let hid_result = headers_utils::get_header_id(headers, "header_id");

    if hid_result.is_err() {
        let err_str = hid_result.err().unwrap();
        let err_result: Result<Vec<tab_organize_info::Model>, Option<String>> = Result::Err(err_str);
        return (StatusCode::OK, Json(WarpResponse::build(err_result)));
    }

    let resp = oranize_service::get_organize_info(hid_result.unwrap()).await;
    (StatusCode::OK, Json(WarpResponse::build(resp)))
}
/**
 *
 */
async fn create_organize(headers: HeaderMap,Json(payload): Json<CreateOrganizeReq>) -> impl IntoResponse {
    let member_id = headers_utils::get_header_id(headers, "header_id").unwrap();
    let resp = oranize_service::create_organize(member_id,payload).await;
    (StatusCode::OK, Json(WarpResponse::build(resp)))
}
