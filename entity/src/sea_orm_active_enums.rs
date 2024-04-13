//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "country_name")]
pub enum CountryName {
    #[sea_orm(string_value = "SPAIN")]
    Spain,
    #[sea_orm(string_value = "SWEDEN")]
    Sweden,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "house_types")]
pub enum HouseTypes {
    #[sea_orm(string_value = "CONDOMINIUM")]
    Condominium,
    #[sea_orm(string_value = "ESTATES_AND_FARMS")]
    EstatesAndFarms,
    #[sea_orm(string_value = "LAND")]
    Land,
    #[sea_orm(string_value = "OTHER_HOUSES")]
    OtherHouses,
    #[sea_orm(string_value = "TOWNHOUSE")]
    Townhouse,
    #[sea_orm(string_value = "VACATION_HOME")]
    VacationHome,
    #[sea_orm(string_value = "VILLA")]
    Villa,
}