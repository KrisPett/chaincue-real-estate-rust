use std::future::Future;
use std::io::Error;
use std::sync::{Arc, Mutex, MutexGuard};

use actix_web::{get, HttpResponse, post, web};
use sea_orm::{ActiveEnum, DatabaseConnection};
use serde::{Deserialize, Serialize};

use entity::countries::Model as Country;
use entity::houses::Model as House;

use crate::AppState;
use crate::helpers::dto_builder_helpers::{country_helper, house_helper};
use crate::services::house_service;
use crate::services::house_service::HouseServiceI;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddPropertyPageDTO {
    id: String,
}

#[derive(Debug, Clone)]
struct DTOBuilder {
    id: String,
}

#[derive(Debug, Deserialize)]
struct CreatePropertyReqBody {
    title: String,
    description: String,
    supply: String,
}

#[post("/add-property")]
pub async fn get_add_property_page(data: web::Data<AppState>, body: web::Json<CreatePropertyReqBody>) -> Result<HttpResponse, Error> {
    log::info!("get_add_property_page");
    let dbc = Arc::new(data.dbc.clone());
    let dto = build_dto(&dbc, &body, |_builder| async { Ok(()) }).await?;
    Ok(HttpResponse::Ok().json(dto))
}

async fn build_dto<F, Fut>(dbc: &Arc<DatabaseConnection>, body: &web::Json<CreatePropertyReqBody>, additional_processing: F) -> Result<AddPropertyPageDTO, Error>
    where
        F: FnOnce(&Arc<Mutex<DTOBuilder>>) -> Fut,
        Fut: Future<Output=Result<(), Error>>,
{
    let dto_builder = Arc::new(Mutex::new(DTOBuilder {
        id: String::new()
    }));

    additional_processing(&dto_builder).await?;

    house_service::HouseService.create_house(dbc, &body.title, &body.description).await?;

    let dto_builder_lock = dto_builder.lock().unwrap();
    Ok(to_home_page_dto(dto_builder_lock.clone()))
}

fn to_home_page_dto(dto_builder: DTOBuilder) -> AddPropertyPageDTO {
    AddPropertyPageDTO {
        id: String::from("id")
    }
}
