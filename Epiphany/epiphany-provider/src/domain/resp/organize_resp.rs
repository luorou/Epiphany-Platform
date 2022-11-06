use epiphany_entity::tab_organize_info;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OrganizeResp {
    pub organize_id: i64,
    pub member_id: i64,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub link_email: Option<String>,
    pub address: Option<String>,
    pub business_license_id: Option<String>,
    pub link_qq: Option<String>,
    pub organize_type: i32,
    pub status: i32,
    pub logo: Option<String>,
    pub business_license_id_file: Option<String>,
    pub create_time: i64,
}

impl OrganizeResp {
    pub fn am_to_resp(mut am: tab_organize_info::ActiveModel) -> Self {
        OrganizeResp {
            organize_id:am.organize_id.take().unwrap(),
            member_id:am.member_id.take().unwrap(),
            name:am.name.take().unwrap(),
            phone:am.phone.take().unwrap(),
            address:am.address.take().unwrap(),
            business_license_id:am.business_license_id.take().unwrap(),
            link_email:am.link_email.take().unwrap(),
            link_qq:am.link_qq.take().unwrap(),
            organize_type:am.organize_type.take().unwrap(),
            status:am.status.take().unwrap(),
            logo:am.logo.take().unwrap(),
            business_license_id_file:am.business_license_id_file.take().unwrap(),
            create_time:am.create_time.take().unwrap(),
        }
    }

    pub fn tab_to_resp( am: tab_organize_info::Model) -> Self {
        OrganizeResp {
            organize_id:am.organize_id,
            member_id:am.member_id,
            name:am.name,
            phone:am.phone,
            address:am.address,
            business_license_id:am.business_license_id,
            link_email:am.link_email,
            link_qq:am.link_qq,
            organize_type:am.organize_type,
            status:am.status,
            logo:am.logo,
            business_license_id_file:am.business_license_id_file,
            create_time:am.create_time
        }
    }
}
