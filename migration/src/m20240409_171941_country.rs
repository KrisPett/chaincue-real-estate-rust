use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;

use crate::extension::postgres::Type;
use crate::m20240410_170204_house::HouseTypes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("country_name"))
                    .values(CountryName::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Countries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Countries::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Countries::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Countries::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Countries::CountryName).enumeration(Alias::new("country_name"), CountryName::iter()))
                    .to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Countries {
    Table,
    Id,
    CountryName,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
pub enum CountryName {
    #[iden = "SWEDEN"]
    SWEDEN,
    #[iden = "SPAIN"]
    SPAIN,
}
