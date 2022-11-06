use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateMenuReq {
    pub code: String,
    pub lebel_zh: String,
    pub lebel_en: String,
    pub icon: String,
    pub level: i32,
    pub path: String,
    pub parent_id: i64,
    
}
