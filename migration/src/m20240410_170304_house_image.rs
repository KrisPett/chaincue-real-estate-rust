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
                    .table(HouseImages::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(HouseImages::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(HouseImages::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(HouseImages::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(HouseImages::Url).text())
                    .col(ColumnDef::new(HouseImages::HouseId).text().not_null())
                    .foreign_key(ForeignKey::create()
                        .from(HouseImages::Table, HouseImages::HouseId)
                        .to(Houses::Table, Houses::Id)
                        .on_delete(ForeignKeyAction::Cascade))
                    .to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum HouseImages {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Url,
    HouseId,
}
