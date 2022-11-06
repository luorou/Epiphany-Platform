//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "tab_admin")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(primary_key)]
    pub admin_id: i64,
    #[sea_orm(primary_key)]
    pub account: String,
    #[serde(skip)]
    pub password: String,
    pub create_time: i64,
    pub delete_time: Option<i64>,
    pub status: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    
}


impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {

}
