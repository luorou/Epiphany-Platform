use axum::{Router, routing::post, response::IntoResponse, Json, http::StatusCode};

use crate::{domain::req::member_req::CreateMemberReq, service::member};


pub fn init_router() -> Router {
    Router::new().route("/register_by_phone", post(register_by_phone))
}

/**
 * register_by_phone
*/
async fn register_by_phone(Json(payload): Json<CreateMemberReq>) -> impl IntoResponse {
    let resp = member::create_member(payload).await;
    (StatusCode::OK,Json(resp))
}
