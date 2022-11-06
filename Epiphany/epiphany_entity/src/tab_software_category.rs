use sea_orm::entity::prelude::*;
use sea_orm::{ RelationTrait};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "tab_software_category")]
pub struct Model {
    #[serde(skip)]
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(primary_key)]
    pub cate_id: i64,
    pub name: String,
    pub parent_id: i64,
    pub level: i64,
    pub cate_type:i64,
}


#[derive(Copy, Clone, Debug, EnumIter,DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "Entity", from = "Column::CateId", to = "Column::ParentId")]
    SelfReferencing,
}


pub struct SelfReferencingLink;

impl Linked for SelfReferencingLink {
    type FromEntity = Entity;

    type ToEntity = Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::SelfReferencing.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

