use std::io::Error;

use futures::TryFutureExt;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use entity::brokers::Model as BrokerModel;
use entity::house_images::Model as HouseImagesModel;
use entity::houses::Model as HouseModel;
use entity::prelude::Houses;
use entity::sea_orm_active_enums::HouseTypes;

use crate::helpers::entity_helper;
use crate::middlewares::errors::CustomErrors;

pub trait HouseServiceI {
    async fn create_house(&self, dbc: &DatabaseConnection, title: &String, description: &String) -> Result<HouseModel, Error>;
    async fn find_all_houses(&self, db_conn: &DatabaseConnection) -> Result<Vec<HouseModel>, Error>;
    async fn find_house_by_id(&self, db_conn: &DatabaseConnection, id: &String) -> Result<Option<HouseModel>, Error>;
    async fn find_broker_by_house_id(&self, db_conn: &DatabaseConnection, house_id: &String) -> Result<Option<BrokerModel>, Error>;
    async fn find_house_images_by_house_id(&self, db_conn: &DatabaseConnection, house_id: &String) -> Result<Vec<HouseImagesModel>, Error>;
}

pub struct HouseService;

impl HouseServiceI for HouseService {
    async fn create_house(&self, dbc: &DatabaseConnection, title: &String, description: &String) -> Result<HouseModel, Error> {
        let house = entity_helper::new_house(String::from(title), String::new(), String::from(description), String::new(), String::new(), String::new(), 0, HouseTypes::Condominium);
        house.insert(dbc)
            .await
            .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))
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
