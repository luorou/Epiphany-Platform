use serde::{Serialize, Deserialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginReq {
    pub account: String,
    pub password: String,
}