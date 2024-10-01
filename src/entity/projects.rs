use sea_orm::prelude::*;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")] // Nom de la table
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}