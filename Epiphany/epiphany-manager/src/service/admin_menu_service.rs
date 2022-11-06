use chrono::Local;
use epiphany_entity::{ tab_admin_menu};
use epiphany_utls::sony_flake_utils;
use sea_orm::{ DatabaseConnection, EntityTrait, ModelTrait, Set, ActiveModelTrait};

use crate::{
    domain::{
        req::admin_menu_req::CreateMenuReq, resp::admin_menu_resp::AdminMenuResp,
    },
    APP_CONTEXT,
};
/**
 * 
 */
pub async fn all() -> Result<Vec<AdminMenuResp>, String> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_admin_menu::Entity::find()
        .all(conn)
        .await;
    if db_result.is_err() {
        return Result::Err(db_result.err().unwrap().to_string());
    }
    let vms = db_result.unwrap();
    let mut resps = Vec::new();
    for ele in vms {
        let childs = ele
            .find_linked(tab_admin_menu::SelfReferencingLink)
            .all(conn)
            .await;
        let resp = AdminMenuResp::tab_child_to_resp(ele, childs.unwrap());
        resps.push(resp);
    }
    return Result::Ok(resps);
}
/**
 * 
 */
pub async fn create_menu(payload: CreateMenuReq) -> Result<AdminMenuResp, String> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    //
    let act = tab_admin_menu::ActiveModel {
        menu_id: Set(sony_flake_utils::generated_id()),
        lebel_zh: Set(payload.lebel_zh),
        lebel_en: Set(payload.lebel_en),
        icon: Set(payload.icon),
        path: Set(payload.path),
        parent_id: Set(payload.parent_id),
        level:Set(payload.level),
        code:Set(payload.code),
        create_time: Set(Local::now().timestamp_millis()),
        ..Default::default()
    }
    .save(conn)
    .await;
    match act {
        Ok(v) => {
            return Result::Ok(AdminMenuResp::am_to_resp(v));
        }
        Err(e) => {
            return Result::Err(e.to_string());
        }
    }
}
