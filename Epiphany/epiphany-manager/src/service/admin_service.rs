use crate::{
    domain::{req::admin_req::CreateAdminReq, resp::admin_resp::AdminResp},
    APP_CONTEXT,
};
use chrono::Local;
use epiphany_entity::tab_admin;
use epiphany_utls::{password_utils, sony_flake_utils};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub async fn get_all_adminer() -> Result<Vec<tab_admin::Model>, String> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_admin::Entity::find().all(conn).await;

    match db_result {
        Ok(rs) => Result::Ok(rs),
        Err(e) => Result::Err(e.to_string()),
    }
}
/**
 * 
 */
pub async fn create_adminer(
    payload: CreateAdminReq,
) -> Result<AdminResp, Option<String>> {

    //
    let password = password_utils::password_balloon_scrypt(payload.password);
    if password.is_err() {
        return Result::Err(password.err().unwrap());
    }
    //
    let conn = APP_CONTEXT.get::<DatabaseConnection>();

    let act = tab_admin::ActiveModel {
        admin_id: Set(sony_flake_utils::generated_id()),
        account: Set(payload.account),
        password: Set(password.unwrap()),
        create_time: Set(Local::now().timestamp_millis()),
        status: Set(0),
        ..Default::default()
    }
    .save(conn)
    .await;
    match act {
        Ok(a) => {
            let model = AdminResp::acm_to_resp(a);
            Result::Ok(model)
        }
        Err(e) => Result::Err(Some(e.to_string())),
    }
}
