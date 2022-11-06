use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use epiphany_utls::warp_response;

use crate::{
    domain::req::member_req::CreateMemberReq,
    service::{ member_service},
};

pub fn init_router() -> Router {
    Router::new().route("/register_by_email", post(register_by_email))
}

/**
 * register_by_phone
*/
async fn register_by_email(Json(payload): Json<CreateMemberReq>) -> impl IntoResponse {
    let resp = member_service::create_member(payload).await;
    (StatusCode::OK, Json(warp_response::WarpResponse::build(resp)))
}
