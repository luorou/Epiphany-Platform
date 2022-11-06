//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tab_provider_member")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub member_id: i64,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub header_url: Option<String>,
    pub status: Option<i32>,
    pub create_time: i64,
    pub delete_time: Option<i64>,
    pub update_time: Option<i64>,
    pub organize_status: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
