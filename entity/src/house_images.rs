//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "house_images")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(column_type = "Text", nullable)]
    pub url: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub house_id: String,
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