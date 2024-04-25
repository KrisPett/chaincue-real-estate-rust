use std::io::Error;

use futures::TryFutureExt;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use entity::brokers::Model as BrokerModel;
use entity::countries;
use entity::countries::Model as CountryModel;
use entity::house_images::Model as HouseImagesModel;
use entity::houses::Model as HouseModel;
use entity::prelude::Houses;
use entity::sea_orm_active_enums::CountryName;

use crate::middlewares::errors::CustomErrors;

pub trait HouseServiceI {
    async fn insert_country(&self, dbc: &DatabaseConnection) -> Result<CountryModel, Error>;
    async fn find_all_houses(&self, db_conn: &DatabaseConnection) -> Result<Vec<HouseModel>, Error>;
    async fn find_house_by_id(&self, db_conn: &DatabaseConnection, id: &String) -> Result<Option<HouseModel>, Error>;
    async fn find_broker_by_house_id(&self, db_conn: &DatabaseConnection, house_id: &String) -> Result<Option<BrokerModel>, Error>;
    async fn find_house_images_by_house_id(&self, db_conn: &DatabaseConnection, house_id: &String) -> Result<Vec<HouseImagesModel>, Error>;
}

pub struct HouseService;

impl HouseServiceI for HouseService {
    async fn insert_country(&self, dbc: &DatabaseConnection) -> Result<CountryModel, Error> {
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

    async fn find_all_houses(&self, db_conn: &DatabaseConnection) -> Result<Vec<HouseModel>, Error> {
        Houses::find().all(db_conn).await.map_err(|err| Error::from(CustomErrors::DatabaseError(err)))
    }

    async fn find_house_by_id(&self, db_conn: &DatabaseConnection, id: &String) -> Result<Option<HouseModel>, Error> {
        Houses::find_by_id(id).one(db_conn).await.map_err(|err| Error::from(CustomErrors::DatabaseError(err)))
    }

    async fn find_broker_by_house_id(&self, db_conn: &DatabaseConnection, house_id: &String) -> Result<Option<BrokerModel>, Error> {
        entity::brokers::Entity::find()
            .filter(entity::brokers::Column::HouseId.eq(house_id))
            .one(db_conn)
            .await
            .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))
    }

    async fn find_house_images_by_house_id(&self, db_conn: &DatabaseConnection, house_id: &String) -> Result<Vec<HouseImagesModel>, Error> {
        entity::house_images::Entity::find()
            .filter(entity::house_images::Column::HouseId.eq(house_id))
            .all(db_conn)
            .await
            .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))
    }
}
