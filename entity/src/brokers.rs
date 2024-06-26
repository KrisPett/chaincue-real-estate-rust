//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "brokers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub house_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::houses::Entity",
        from = "Column::HouseId",
        to = "super::houses::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Houses,
}

impl Related<super::houses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Houses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
