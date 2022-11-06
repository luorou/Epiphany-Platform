use crate::middleware::auth_admin;
use crate::{domain::req::admin_req::CreateAdminReq, service::admin_service};
use axum::http::StatusCode;
use axum::middleware::from_extractor;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use epiphany_utls::warp_response::WarpResponse;

pub fn init_router() -> Router {
    auth_router().merge(un_auth_router())
}

/**
 */
fn auth_router() -> Router {
    Router::new()
        .route("/get_adaminers", post(get_adaminers))
        .layer(from_extractor::<auth_admin::AuthAdmin>())
}

fn un_auth_router() -> Router {
    Router::new().route("/create_adminer", post(create_adminer))
}

/**
 * controller  get all adminer  
*/
async fn get_adaminers() -> impl IntoResponse {
    let db_result = admin_service::get_all_adminer().await;
    match db_result {
        Ok(m) => (StatusCode::OK, Json(WarpResponse::build(Ok(m)))),
        Err(e) => (
            StatusCode::OK,
            Json(WarpResponse::build(Err(Some(e.to_string())))),
        ),
    }
}
/**
 * create adminer 
 */
async fn create_adminer(Json(payload): Json<CreateAdminReq>) -> impl IntoResponse {
    let resp = admin_service::create_adminer(payload).await;

    (StatusCode::OK, Json(WarpResponse::build(resp)))
}
