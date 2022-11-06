use epiphany_utls::{ sony_flake_utils};
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    domain::{req::software_info_req::CreateAppInfoReq, resp::software_info_resp::AppInfoResp},
    APP_CONTEXT,
};

pub async fn create_app_info(
    mut payload: CreateAppInfoReq,
 ) -> Result<AppInfoResp, Option<String>> {
     let conn = APP_CONTEXT.get::<DatabaseConnection>();
     payload.software_id = sony_flake_utils::generated_id();
     let act =payload.req_to_am().save(conn).await;
     match act {
         Ok(v) => {
             return Result::Ok(AppInfoResp::am_to_resp(v));
         }
         Err(e) => {
             return Result::Err(Some(e.to_string()));
         }
     }
 }