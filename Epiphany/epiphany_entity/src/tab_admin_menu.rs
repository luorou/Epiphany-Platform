use sea_orm::entity::prelude::*;
use sea_orm::{ RelationTrait};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,serde::Serialize,serde::Deserialize)]
#[sea_orm(table_name = "tab_admin_menu")]
pub struct Model {
    #[serde(skip)]
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(primary_key)]
    pub menu_id: i64,
    pub code: String,
    pub lebel_zh: String,
    pub lebel_en: String,
    pub icon: String,
    pub level: i32,
    pub path: String,
    pub parent_id: i64,
    pub create_time: i64,
    pub delete_time: Option<i64>,
    pub update_time: Option<i64>,
    pub status: i32,
}

#[derive(Copy, Clone, Debug, EnumIter,DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "Entity", from = "Column::MenuId", to = "Column::ParentId")]
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
