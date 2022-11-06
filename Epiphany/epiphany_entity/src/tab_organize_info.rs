//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,serde::Deserialize,serde::Serialize)]
#[sea_orm(table_name = "tab_organize_info")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[serde(skip)]
    pub id: i64,
    pub organize_id:i64,
    pub member_id:i64,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub business_license_id: Option<String>,
    pub link_email: Option<String>,
    pub link_qq: Option<String>,
    pub organize_type: i32,
    pub create_time: i64,
    #[serde(skip)]
    pub update_time: Option<i64>,
    #[serde(skip)]
    pub delete_time: Option<i64>,
    pub status: i32,
    pub logo: Option<String>,
    pub business_license_id_file: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}