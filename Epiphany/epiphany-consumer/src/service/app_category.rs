use epiphany_entity::tab_software_category;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

use crate::{
    domain::{resp::app_category_resp::AppCategotyResp, req::app_category_req::FetchCategotyReq,
    },
    APP_CONTEXT,
};
/**
 * 
 */
pub async fn all(args:FetchCategotyReq) -> Result<Vec<AppCategotyResp>, String> {
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