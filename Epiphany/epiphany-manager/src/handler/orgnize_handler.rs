use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use epiphany_utls::{ warp_response::WarpResponse};
use serde::{Deserialize, Serialize};
use crate::{service::company_service};

pub fn init_router() -> Router {
    Router::new()
        .route("/get_orgnize", post(get_company))
}
#[derive(Deserialize,Serialize)]
struct SomeRequest{
    status:Option<i32>
}
/**
 * 
 */
async fn get_company(Json(payload): Json<SomeRequest>) -> impl IntoResponse {
    let status = payload.status;
    let resp = company_service::get_company(status).await;
    (StatusCode::OK,Json(WarpResponse::build(resp)))
    
}
