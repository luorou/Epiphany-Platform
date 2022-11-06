use epiphany_entity::{tab_organize_info};
use epiphany_utls::{sony_flake_utils};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    domain::{req::organize_req::CreateOrganizeReq, resp::organize_resp::OrganizeResp},
    APP_CONTEXT,
};
/**
 * 
 */
pub async fn create_organize(
    member_id:i64,
    mut payload: CreateOrganizeReq,
) -> Result<OrganizeResp, Option<String>> {
    
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    //
    let haved = tab_organize_info::Entity::find()
        .filter(tab_organize_info::Column::MemberId.eq(member_id.clone()))
        .all(conn)
        .await;
    //
    if haved.is_err() {
        return Result::Err(Some(haved.err().unwrap().to_string()));
    }
    //
    if !haved.unwrap().is_empty() {
        return Result::Err(Some(String::from("has been registed")));
    }
    payload.member_id = member_id;
    //
    payload.organize_id = sony_flake_utils::generated_id();
    let act = payload.req_to_am().save(conn).await;
    match act {
        Ok(v) => {
            return Result::Ok(OrganizeResp::am_to_resp(v));
        }
        Err(e) => {
            return Result::Err(Some(e.to_string()));
        }
    }
}


/**
 * 
 */
pub async fn get_organize_info(id: i64) -> Result<Vec<tab_organize_info::Model>, Option<String>> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_organize_info::Entity::find()
        .filter(tab_organize_info::Column::MemberId.eq(id))
        .all(conn)
        .await;

    match db_result {
        Ok(v) => Result::Ok(v),
        Err(e) => Result::Err(Some(e.to_string())),
    }
}
