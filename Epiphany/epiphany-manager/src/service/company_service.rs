use epiphany_entity::tab_organize_info;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::APP_CONTEXT;

pub async fn get_company(status: Option<i32>) -> Result<Vec<tab_organize_info::Model>, Option<String>> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_req = match status {
        Some(v) => tab_organize_info::Entity::find().filter(tab_organize_info::Column::Status.eq(v)),
        None => tab_organize_info::Entity::find(),
    };
    let db_result = db_req.all(conn).await;
    match db_result {
        Ok(v) => Result::Ok(v),
        Err(e) => Result::Err(Some(e.to_string())),
    }
}
