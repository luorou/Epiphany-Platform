use epiphany_entity::tab_software_category;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AppCategotyResp {
    pub cate_id: i64,
    pub name: String,
    pub parent_id: i64,
    pub level: i64,
    pub child: Option<Vec<tab_software_category::Model>>,
}

impl AppCategotyResp {
    pub fn tab_to_resp(args: tab_software_category::Model) -> Self {
        AppCategotyResp {
            cate_id: args.cate_id,
            name: args.name,
            parent_id: args.parent_id,
            level: args.level,
            child: None,
        }
    }

    pub fn tab_child_to_resp(
        parent: tab_software_category::Model,
        child: Vec<tab_software_category::Model>,
    ) -> Self {
        AppCategotyResp {
            cate_id: parent.cate_id,
            name: parent.name,
            parent_id: parent.parent_id,
            level: parent.level,
            child: Some(child),
        }
    }

    pub fn am_to_resp(mut am:tab_software_category::ActiveModel)->Self{
        AppCategotyResp{
            cate_id:am.cate_id.take().unwrap(),
            name: am.name.take().unwrap(),
            parent_id: am.parent_id.take().unwrap(),
            level: am.level.take().unwrap(),
            child: None,
        }
    }
}
