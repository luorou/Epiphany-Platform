

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use epiphany_utls::warp_response;

use crate::{
    domain::req::software_info_req::CreateAppInfoReq,
    service::{app_info_service},
};

pub fn init_router() -> Router {
    Router::new().route("/create_app", post(create_app))
}

/**
 * register_by_phone
*/
async fn create_app(Json(payload): Json<CreateAppInfoReq>) -> impl IntoResponse {
    let resp = app_info_service::create_app_info(payload).await;
    (StatusCode::OK, Json(warp_response::WarpResponse::build(resp)))
}