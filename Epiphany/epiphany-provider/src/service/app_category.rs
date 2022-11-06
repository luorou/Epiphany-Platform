use epiphany_entity::tab_software_category;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

use crate::{
    domain::{resp::software_category_resp::AppCategotyResp,
    },
    APP_CONTEXT,
};
/**
 * 
 */
pub async fn all() -> Result<Vec<AppCategotyResp>, String> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_software_category::Entity::find()
        .filter(tab_software_category::Column::ParentId.eq(427362848532866690 as i64))
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