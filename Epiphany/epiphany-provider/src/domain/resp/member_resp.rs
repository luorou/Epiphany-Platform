use epiphany_entity::{tab_provider_member};


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MemberResp {
    pub member_id:i64,
    pub phone: Option<String>,
    pub email:Option<String>,
    pub header_url: Option<String>,
    pub status: Option<i32>,
    pub organize_status: i32,
}

impl MemberResp{
    pub fn am_to_resp(mut args:tab_provider_member::ActiveModel)->Self{
        MemberResp{
            member_id:args.member_id.take().unwrap(),
            phone:args.phone.take().unwrap(),
            email:args.email.take().unwrap(),
            header_url:args.header_url.take().unwrap(),
            status:args.status.take().unwrap(),
            organize_status:args.organize_status.take().unwrap(),
        } 
    }

    pub fn tab_to_resp( args:tab_provider_member::Model)->Self{
        MemberResp{
            member_id:args.member_id,
            phone:args.phone,
            email:args.email,
            header_url:args.header_url,
            status:args.status,
            organize_status:args.organize_status
        } 
    }
}