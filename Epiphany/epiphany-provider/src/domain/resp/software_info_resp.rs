use epiphany_entity::tab_software_info;


#[derive(Clone, Debug, serde::Serialize,serde::Deserialize)]
pub struct AppInfoResp {
    pub software_id: i64,
    pub app_name: String,
    pub app_logo: String,
    pub app_platform: i32,
    pub app_pack_name: String,
    pub orgnize_id: i64,
    pub privacy_protocal_url: String,
    pub status: i32,
}

impl AppInfoResp {
    pub fn am_to_resp(mut am: tab_software_info::ActiveModel) -> Self {
        AppInfoResp {
            software_id:am.software_id.take().unwrap(),
            app_name:am.app_name.take().unwrap(),
            app_logo:am.app_logo.take().unwrap(),
            app_platform:am.app_platform.take().unwrap(),
            app_pack_name:am.app_pack_name.take().unwrap(),
            orgnize_id:am.orgnize_id.take().unwrap(),
            privacy_protocal_url:am.privacy_protocal_url.take().unwrap(),
            status:am.status.take().unwrap(),
        }
    }
}