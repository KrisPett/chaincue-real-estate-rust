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
                    .col(ColumnDef::new(Houses::Id).string_len(255).not_null().primary_key())
                    .col(ColumnDef::new(Houses::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Houses::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Houses::Title).string_len(255))
                    .col(ColumnDef::new(Houses::Description).string_len(10000))
                    .col(ColumnDef::new(Houses::Location).string_len(255))
                    .col(ColumnDef::new(Houses::Country).string_len(255))
                    .col(ColumnDef::new(Houses::City).string_len(255))
                    .col(ColumnDef::new(Houses::NumberRooms).big_integer())
                    .col(ColumnDef::new(Houses::Beds).big_integer())
                    .col(ColumnDef::new(Houses::Price).big_integer())
                    .col(ColumnDef::new(Houses::Src).string_len(255))
                    .col(ColumnDef::new(Houses::Sold).boolean())
                    .col(ColumnDef::new(Houses::HouseTypes).enumeration(Alias::new("house_types"), HouseTypes::iter()).not_null())
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
