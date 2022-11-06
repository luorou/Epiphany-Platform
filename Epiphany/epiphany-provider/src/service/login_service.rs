use epiphany_entity::tab_provider_member;
use epiphany_utls::{password_utils, jwt_utils};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::domain::req::login_req::{LoginByEmailReq,LoginByPhoneReq};
use crate::APP_CONTEXT;

use super::redis_service::set_string;

/**
 *
 */
pub async fn login_by_email(payload: LoginByEmailReq) -> Result<String, Option<String>> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_provider_member::Entity::find()
        .filter(tab_provider_member::Column::Email.eq(payload.email))
        .one(conn)
        .await;
    if db_result.is_err() {
        return Result::Err(Some(db_result.err().unwrap().to_string()));
    }
    let db_model = db_result.unwrap();
    if db_model.is_none() {
        return Result::Err(Some(String::from("member is not exitst")));
    }
    let model = db_model.unwrap();
    if !password_utils::password_balloon_eq(payload.password, model.password) {
        return Result::Err(Some(String::from("password  wrong")));
    }
    let token_result = jwt_utils::create_token(model.member_id);
    if token_result.is_err() {
        return Result::Err(Some(token_result.err().unwrap().unwrap()));
    }
    let _ = set_string(
        model.member_id.to_string().as_str(),
        token_result.clone().unwrap().as_str(),
    );
    return Result::Ok(token_result.unwrap());
}


pub async fn login_by_phone(payload: LoginByPhoneReq) -> Result<String, Option<String>> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_provider_member::Entity::find()
        .filter(tab_provider_member::Column::Phone.eq(payload.phone))
        .one(conn)
        .await;
    if db_result.is_err() {
        return Result::Err(Some(db_result.err().unwrap().to_string()));
    }
    let db_model = db_result.unwrap();
    if db_model.is_none() {
        return Result::Err(Some(String::from("member is not exitst")));
    }
    let model = db_model.unwrap();
    if !password_utils::password_balloon_eq(payload.password, model.password) {
        return Result::Err(Some(String::from("password  wrong")));
    }
    let token_result = jwt_utils::create_token(model.member_id);
    if token_result.is_err() {
        return Result::Err(Some(token_result.err().unwrap().unwrap()));
    }
    let _ = set_string(
        model.member_id.to_string().as_str(),
        token_result.clone().unwrap().as_str(),
    );
    return Result::Ok(token_result.unwrap());
}