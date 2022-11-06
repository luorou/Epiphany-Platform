use epiphany_entity::tab_admin;
use serde::{Serialize, Deserialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminResp{
    pub admin_id: i64,
    pub account: String,
    pub create_time: i64,
    pub delete_time: Option<i64>,
    pub status: i32,
}

impl AdminResp{
    //
    pub fn acm_to_resp(mut am:tab_admin::ActiveModel)->Self{
        AdminResp{
            admin_id:am.admin_id.take().unwrap(),
            account:am.account.take().unwrap(),
            create_time:am.create_time.take().unwrap(),
            delete_time:am.delete_time.take().unwrap(),
            status:am.status.take().unwrap()
        }
    }
}