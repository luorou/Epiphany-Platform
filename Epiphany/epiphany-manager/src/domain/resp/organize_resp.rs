use epiphany_entity::tab_organize_info;

#[derive(Clone,serde::Deserialize,serde::Serialize)]
pub struct OrganizeResp {
    pub organize_id: i64,
    pub member_id: i64,
    pub name: String,
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

impl OrganizeResp{
      pub fn tab_to_resp(args: tab_organize_info::Model) -> Self {
        OrganizeResp {
            organize_id:args.organize_id,
            member_id:args.member_id,
            name:args.name,
            phone:args.phone,
            address:args.address,
            business_license_id:args.business_license_id,
            link_email:args.link_email,
            link_qq:args.link_qq,
            organize_type:args.organize_type,
            status:args.status,
            logo:args.logo,
            business_license_id_file:args.business_license_id_file,
            create_time:args.create_time
        }
    }
    
}