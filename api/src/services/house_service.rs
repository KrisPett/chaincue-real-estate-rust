use std::io::Error;

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use entity::countries;
use entity::countries::Model;
use entity::houses::Model as House;
use entity::prelude::Houses;
use entity::sea_orm_active_enums::CountryName;

use crate::middlewares::errors::CustomErrors;

pub async fn insert(dbc: &DatabaseConnection) -> Result<Model, Error> {
    let country = countries::ActiveModel {
        id: Set(String::from(Uuid::new_v4())),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        country_name: Set(CountryName::Spain),
    };

    match country.insert(dbc).await {
        Ok(model) => Ok(model),
        Err(err) => Err(Error::from(CustomErrors::DatabaseError(err)))
    }
}

pub async fn find_all(db_conn: &DatabaseConnection) -> Result<Vec<House>, Error> {
    let houses = Houses::find()
        .all(db_conn)
        .await
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;
    Ok(houses)
}

pub async fn find_by_id(db_conn: &DatabaseConnection, id: String) -> Result<Option<House>, Error> {
    let house = Houses::find_by_id(id)

        // .find_with_related(entity::brokers::Entity, entity::house_images::Entity)
        .one(db_conn)
        .await
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;
    Ok(house)
}
