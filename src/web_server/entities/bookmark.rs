//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bookmark")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub url: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: String,
    pub ingested: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
