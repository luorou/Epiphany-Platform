use serde::{Serialize, Deserialize};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct CreateMemberReq{
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub header_url: Option<String>,
}