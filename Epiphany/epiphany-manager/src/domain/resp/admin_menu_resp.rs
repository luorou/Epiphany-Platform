use epiphany_entity::tab_admin_menu;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminMenuResp {
    pub menu_id: i64,
    pub code: String,
    pub lebel: String,
    pub icon: String,
    pub level: i32,
    pub path: String,
    pub parent_id: i64,
    pub children: Option<Vec<tab_admin_menu::Model>>,
}

impl AdminMenuResp {
    //
    pub fn am_to_resp(mut am: tab_admin_menu::ActiveModel) -> Self {
        AdminMenuResp {
            menu_id: am.menu_id.take().unwrap(),
            code: am.code.take().unwrap(),
            lebel: am.lebel_zh.take().unwrap(),
            icon: am.icon.take().unwrap(),
            path: am.path.take().unwrap(),
            parent_id: am.parent_id.take().unwrap(),
            level: am.level.take().unwrap(),
            children: None,
        }
    }

    pub fn tab_child_to_resp(
        args: tab_admin_menu::Model,
        child: Vec<tab_admin_menu::Model>,
    ) -> Self {
        AdminMenuResp {
            menu_id: args.menu_id,
            code: args.code,
            lebel: args.lebel_zh,
            icon: args.icon,
            path: args.path,
            parent_id: args.parent_id,
            level: args.level,
            children: Some(child),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuLebel {
    pub zh_cn: String,
    pub en_us: String,
}
