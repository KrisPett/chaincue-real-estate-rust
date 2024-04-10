use sea_orm_migration::prelude::*;
use crate::m20240410_170204_house::Houses;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Brokers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Brokers::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Brokers::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Brokers::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Brokers::Name).text())
                    .col(ColumnDef::new(Brokers::PhoneNumber).text())
                    .col(ColumnDef::new(Brokers::Email).text())
                    .col(ColumnDef::new(Brokers::HouseId).text().not_null())
                    .foreign_key(ForeignKey::create()
                        .from(Brokers::Table, Brokers::HouseId)
                        .to(Houses::Table, Houses::Id)
                        .on_delete(ForeignKeyAction::Cascade))
                    .to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Brokers {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Name,
    PhoneNumber,
    Email,
    HouseId,
}
