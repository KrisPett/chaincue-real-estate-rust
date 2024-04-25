use std::io::Error;

use futures::TryFutureExt;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use entity::brokers::Model as Broker;
use entity::countries;
use entity::countries::Model;
use entity::house_images::Model as HouseImages;
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
    let (house, images, broker) = tokio::try_join!(
        Houses::find_by_id(&id).one(db_conn)
            .map_err(|err| Error::from(CustomErrors::DatabaseError(err))),
        find_house_images_by_house_id(db_conn, &id),
        find_broker_by_house_id(db_conn, &id)
    )?;

    println!("{:?}", images);
    println!("{:?}", broker);
    println!("{:?}", house);

    Ok(house)
}

pub async fn find_broker_by_house_id(db_conn: &DatabaseConnection, house_id: &String) -> Result<Option<Broker>, Error> {
    let result = entity::brokers::Entity::find()
        .filter(entity::brokers::Column::HouseId.eq(house_id))
        .one(db_conn)
        .await
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;
    Ok(result)
}

pub async fn find_house_images_by_house_id(db_conn: &DatabaseConnection, house_id: &String) -> Result<Vec<HouseImages>, Error> {
    let house_images = entity::house_images::Entity::find()
        .filter(entity::house_images::Column::HouseId.eq(house_id))
        .all(db_conn)
        .await
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;
    Ok(house_images)
}
