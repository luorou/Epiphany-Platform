use chrono::Local;
use epiphany_entity::tab_organize_info;
use sea_orm::Set;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateOrganizeReq {
    #[serde(skip)]
    pub organize_id: i64,
    #[serde(skip)]
    pub member_id: i64,
    pub name: String,
    pub phone: Option<String>,
    pub link_email: Option<String>,
    pub link_qq: Option<String>,
    pub address: Option<String>,
    pub organize_type: i32,
    pub business_license_id: Option<String>,
    pub business_license_id_file: Option<String>,
    pub logo: Option<String>,
}

impl CreateOrganizeReq {
    pub fn req_to_am(self) -> tab_organize_info::ActiveModel {
        tab_organize_info::ActiveModel {
            organize_id: Set(self.organize_id),
            member_id: Set(self.member_id),
            name: Set(self.name),
            phone: Set(self.phone),
            link_email: Set(self.link_email),
            address:Set(self.address),
            link_qq: Set(self.link_qq),
            organize_type: Set(self.organize_type),
            business_license_id: Set(self.business_license_id),
            business_license_id_file: Set(self.business_license_id_file),
            status: Set(0),
            create_time: Set(Local::now().timestamp_millis()),
            ..Default::default()
        }
    }
}
