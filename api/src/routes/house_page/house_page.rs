use std::future::Future;
use std::io::Error;
use std::sync::{Arc, Mutex, MutexGuard};

use actix_web::{get, HttpResponse, web};
use sea_orm::{ActiveEnum, DatabaseConnection};
use serde::{Deserialize, Serialize};

use entity::countries::Model as Country;
use entity::houses::Entity as House;
use entity::brokers::Model as Broker;

use crate::AppState;
use crate::helpers::dto_builder_helpers::{country_helper, house_helper};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HousePageDTO {
    id: String,
    title: String,
    location: String,
    r#type: String,
    number_rooms: i64,
    beds: i64,
    dollar_price: String,
    crypto_price: String,
    src: String,
    images: Vec<ImageDTO>,
    broker: BrokerDTO,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImageDTO {
    id: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrokerDTO {
    id: String,
    name: String,
    phone_number: String,
    email: String,
}

#[derive(Debug, Clone)]
struct DTOBuilder {
    house: entity::houses::Model,
}

#[get("/house/{house_id}")]
pub async fn get_house_page(data: web::Data<AppState>, path: web::Path<String>) -> Result<HttpResponse, Error> {
    log::info!("get_house_page");
    log::info!("{}" ,path);
    let dbc = Arc::new(data.dbc.clone());
    let dto = build_dto(&dbc, |_builder| async { Ok(()) }).await?;
    Ok(HttpResponse::Ok().json(dto))
}

async fn build_dto<F, Fut>(dbc: &Arc<DatabaseConnection>, additional_processing: F) -> Result<HousePageDTO, Error>
    where
        F: FnOnce(&Arc<Mutex<DTOBuilder>>) -> Fut,
        Fut: Future<Output=Result<(), Error>>,
{
    let dto_builder = Arc::new(Mutex::new(DTOBuilder { house: House }));

    additional_processing(&dto_builder).await?;

    let dto_builder_clone = Arc::clone(&dto_builder);
    let dbc_clone = Arc::clone(&dbc);
    house_helper::update_dto_builder_with_house_by_id(&dbc_clone, String::from("id"), |dto_builder_mutex, house| {
        let mut dto_builder: MutexGuard<DTOBuilder> = dto_builder_mutex.lock().unwrap();
        dto_builder.house = house;
    })(&dto_builder_clone).await?;

    let dto_builder_lock = dto_builder.lock().unwrap();
    Ok(to_home_page_dto(dto_builder_lock.clone()))
}


fn to_home_page_dto(dto_builder: DTOBuilder) -> HousePageDTO {
    HousePageDTO {
        id: dto_builder.house.id,
        title: dto_builder.house.title.unwrap_or(String::new()),
        location: dto_builder.house.location.unwrap_or(String::new()),
        r#type: dto_builder.house.house_types.to_value(),
        number_rooms: dto_builder.house.number_rooms.unwrap_or(0),
        beds: dto_builder.house.beds.unwrap_or(0),
        dollar_price: "".to_string(),
        crypto_price: String::from("₿32.346"),
        src: dto_builder.house.src.unwrap_or(String::new()),
        images: vec![],
        broker: to_broker_dto(dto_builder.house.broker),
    }
}

fn to_broker_dto(broker: Broker) -> BrokerDTO {
    BrokerDTO {
        id: broker.id,
        name: broker.name,
        phone_number: broker.phone_number.unwrap_or(String::new()),
        email: broker.email.unwrap_or(String::new()),
    }
}
