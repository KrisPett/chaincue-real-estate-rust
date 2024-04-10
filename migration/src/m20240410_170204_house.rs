use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("house_types"))
                    .values(HouseTypes::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Houses::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Houses::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Houses::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Houses::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Houses::Title).text())
                    .col(ColumnDef::new(Houses::Description).text())
                    .col(ColumnDef::new(Houses::Location).text())
                    .col(ColumnDef::new(Houses::Country).text())
                    .col(ColumnDef::new(Houses::City).text())
                    .col(ColumnDef::new(Houses::NumberRooms).big_integer())
                    .col(ColumnDef::new(Houses::Beds).big_integer())
                    .col(ColumnDef::new(Houses::Price).big_integer())
                    .col(ColumnDef::new(Houses::Src).text().not_null())
                    .col(ColumnDef::new(Houses::Sold).boolean())
                    .col(ColumnDef::new(Houses::HouseTypes).enumeration(Alias::new("house_types"), HouseTypes::iter()))
                    .to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Houses {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Title,
    Description,
    Location,
    Country,
    City,
    NumberRooms,
    Beds,
    Price,
    Src,
    Sold,
    HouseTypes,
}

#[derive(Iden, EnumIter)]
pub enum HouseTypes {
    #[iden = "CONDOMINIUM"]
    CONDOMINIUM,
    #[iden = "VILLA"]
    VILLA,
    #[iden = "TOWNHOUSE"]
    TOWNHOUSE,
    #[iden = "VACATION_HOME"]
    VacationHome,
    #[iden = "ESTATES_AND_FARMS"]
    EstatesAndFarms,
    #[iden = "LAND"]
    LAND,
    #[iden = "OTHER_HOUSES"]
    OtherHouses,
}
