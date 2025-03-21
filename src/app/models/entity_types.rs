//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "entity_types")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_entity_type: i32,
    pub name: String,
    pub allows_breeding: i32,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::entities::Entity")]
    Entities,
}

impl Related<super::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entities.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
