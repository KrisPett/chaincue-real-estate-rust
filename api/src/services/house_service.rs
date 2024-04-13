use actix_web::{Error, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use entity::countries;
use entity::countries::Model;
use entity::sea_orm_active_enums::CountryName;

use crate::utilities::errors::CustomErrors;

pub async fn insert(dbc: &DatabaseConnection) -> Result<Model, Error> {
    let country = countries::ActiveModel {
        id: Set(String::from(Uuid::new_v4())),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        country_name: Set(Some(CountryName::Spain)),
    };

    match country.insert(dbc).await {
        Ok(model) => Ok(model),
        Err(err) => Err(Error::from(CustomErrors::DatabaseError(err))),
    }
}
