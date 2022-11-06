use epiphany_entity::tab_admin;
use epiphany_utls::{password_utils, jwt_utils};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::caches::redis_service::set_string;
use crate::domain::req::login_req::LoginReq;
use crate::APP_CONTEXT;

/**
 *
 */
pub async fn login_by_account(payload: LoginReq) -> Result<String, Option<String>> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_admin::Entity::find()
        .filter(tab_admin::Column::Account.ends_with(&payload.account))
        .one(conn)
        .await;
    if db_result.is_err() {
        return Result::Err(Some(db_result.err().unwrap().to_string()));
    }
    let db_model = db_result.unwrap();
    if db_model.is_none() {
        return Result::Err(Some(String::from("adminer is not exitst")));
    }
    let model = db_model.unwrap();
    if !password_utils::password_balloon_eq(payload.password, model.password) {
        return Result::Err(Some(String::from("password  wrong")));
    }
    let token_result = jwt_utils::create_token(model.admin_id);
    if token_result.is_err() {
        return Result::Err(Some(token_result.err().unwrap().unwrap()));
    }
    let _ = set_string(
        model.admin_id.to_string().as_str(),
        token_result.clone().unwrap().as_str(),
    );
    return Result::Ok(token_result.unwrap());
}
