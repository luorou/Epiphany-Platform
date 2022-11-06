use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAppCategotyReq {
    pub name: String,
    pub parent_id: i64,
    pub cate_type: i64,
    pub level:i64,
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FetchCategotyReq {
    pub cate_type: i64,
    pub level:i64,
}