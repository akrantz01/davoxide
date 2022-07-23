//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, IntoActiveModel};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(name = "User")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub username: String,
    pub name: String,
    #[graphql(skip)]
    pub access_token: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn as_active_model(&self) -> ActiveModel {
        self.clone().into_active_model()
    }
}
