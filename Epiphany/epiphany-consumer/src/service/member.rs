use chrono::Local;
use epiphany_entity::tab_member;
use epiphany_utls::{password_utils, sony_flake_utils};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{
    domain::{req::member_req::CreateMemberReq, resp::member_resp::MemberResp},
    APP_CONTEXT,
};

pub async fn create_member(args: CreateMemberReq) -> Result<MemberResp, Option<String>> {
    let password = password_utils::password_balloon_scrypt(args.password);
    if password.is_err() {
        return Result::Err(password.err().unwrap());
    }
    //
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let act = tab_member::ActiveModel {
        member_id: Set(sony_flake_utils::generated_id()),
        phone: Set(args.phone),
        password: Set(password.unwrap()),
        create_time: Set(Local::now().timestamp_millis()),
        ..Default::default()
    }
    .save(conn)
    .await;
    match act {
        Ok(a) => {
            let m = MemberResp::am_to_resp(a);
            Result::Ok(m)
        }
        Err(e) => Result::Err(Some(e.to_string())),
    }
}

pub async fn get_member_by_id(args: i64) -> Result<MemberResp, Option<String>> {
    let conn = APP_CONTEXT.get::<DatabaseConnection>();
    let db_result = tab_member::Entity::find()
        .filter(tab_member::Column::MemberId.eq(args))
        .one(conn)
        .await;
    match db_result {
        Ok(a) => match a {
            Some(v) => {
                let m = MemberResp::tab_to_resp(v);
                Result::Ok(m)
            }
            None => Result::Err(Some(String::from(""))),
        },
        Err(e) => Result::Err(Some(e.to_string())),
    }
}
