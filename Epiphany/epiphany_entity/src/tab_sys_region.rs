

//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tab_sys_region")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub region_id: i64,
    pub region_name: String,
    pub region_short_name: Option<String>,
    pub region_code: Option<String>,
    pub region_parent_id: Option<i64>,
    pub region_level: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
