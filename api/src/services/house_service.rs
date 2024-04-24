use std::io::{Error, stdout};
use futures::TryFutureExt;

use sea_orm::{ActiveModelTrait, DatabaseConnection, DbBackend, EntityTrait, JoinType, QuerySelect, QueryTrait, Related, Set, RelationTrait, EntityOrSelect};
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use entity::countries;
use entity::countries::Model;
use entity::houses::Model as House;
use entity::prelude::Houses;
use entity::prelude::Brokers;
use entity::prelude::HouseImages;
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

    Houses::find_by_id(&id).one(db_conn)
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)));
    let (house, images, broker) = tokio::try_join!(
        Houses::find_by_id(&id).one(db_conn)
            .map_err(|err| Error::from(CustomErrors::DatabaseError(err))),
        Houses::find_by_id(&id).find_with_related(entity::house_images::Entity).all(db_conn)
            .map_err(|err| Error::from(CustomErrors::DatabaseError(err))),
        Houses::find_by_id(&id).find_with_related(entity::brokers::Entity).all(db_conn)
            .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))
    )?;

    println!("{:?}", images);
    println!("{:?}", broker);
    println!("{:?}", house);

    Ok(house)
}

