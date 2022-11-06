use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use epiphany_utls::{ warp_response};
use crate::domain::req::admin_menu_req::CreateMenuReq;
use crate::{service,};


pub fn init_router() -> Router {
      Router::new()
          .route("/crate_menu", post(crate_menu))
          .route("/list_menu", post(list_menu))
  }
  
  
  /** */
  async fn crate_menu(Json(payload): Json<CreateMenuReq>) -> impl IntoResponse {
      let resp = service::admin_menu_service::create_menu(payload).await;
      (StatusCode::OK,Json(resp))
  }
  
  async fn list_menu() -> impl IntoResponse {
      let db_result = service::admin_menu_service::all().await;
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