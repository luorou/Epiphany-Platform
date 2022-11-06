use epiphany_entity::tab_software_category;
use epiphany_utls::sony_flake_utils;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set, ActiveModelTrait};

use crate::{
    domain::{
        req::app_category_req::{CreateAppCategotyReq, LevelTypeCategryReq}, resp::app_category_resp::AppCategotyResp,
    },
    APP_CONTEXT,
};
/**
 * 
 */
pub async fn all(args:LevelTypeCategryReq) -> Result<Vec<AppCategotyResp>, String> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_software_category::Entity::find()
        .filter(tab_software_category::Column::Level.eq(args.level))
        .filter(tab_software_category::Column::CateType.eq(args.cate_type))
        .all(conn)
        .await;
    if db_result.is_err() {
        return Result::Err(db_result.err().unwrap().to_string());
    }
    let vms = db_result.unwrap();
    let mut resps = Vec::new();
    for ele in vms {
        let childs = ele
            .find_linked(tab_software_category::SelfReferencingLink)
            .all(conn)
            .await;
        let resp = AppCategotyResp::tab_child_to_resp(ele, childs.unwrap());
        resps.push(resp);
    }
    return Result::Ok(resps);
}
/**
 * 
 */
pub async fn create_category(payload: CreateAppCategotyReq) -> Result<AppCategotyResp, String> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
        //
    let act = tab_software_category::ActiveModel {
        cate_id: Set(sony_flake_utils::generated_id()),
        name: Set(payload.name),
        parent_id: Set(payload.parent_id),
        cate_type:Set(payload.cate_type),
        level:Set(payload.level),
        ..Default::default()
    }
    .save(conn)
    .await;
    match act {
        Ok(v) => {
            return Result::Ok(AppCategotyResp::am_to_resp(v));
        }
        Err(e) => {
            return Result::Err(e.to_string());
        }
    }
}
