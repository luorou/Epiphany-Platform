use chrono::Local;
use epiphany_entity::tab_software_info;
use sea_orm::Set;


#[derive(Clone, Debug, serde::Serialize,serde::Deserialize)]
pub struct CreateAppInfoReq {
    #[serde(skip)]
    pub software_id:i64,
    pub app_name: String,
    pub app_logo: String,
    pub app_platform: i32,
    pub app_pack_name: String,
    pub company_id: i64,
    pub privacy_protocal_url: String,
}

impl CreateAppInfoReq {
    pub fn req_to_am(self)->tab_software_info::ActiveModel{
        tab_software_info::ActiveModel{
            software_id:Set(self.software_id),
            app_name:Set(self.app_name),
            app_logo:Set(self.app_logo),
            app_platform:Set(self.app_platform),
            app_pack_name:Set(self.app_pack_name),
            company_id:Set(self.company_id),
            privacy_protocal_url:Set(self.privacy_protocal_url),
            status:Set(0),
            create_time: Set(Local::now().timestamp_millis()),
            ..Default::default()
        }
    }
}