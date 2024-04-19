use std::future::Future;
use std::io::Error;
use std::sync::{Arc, Mutex, MutexGuard};

use actix_web::{get, HttpResponse, web};
use sea_orm::{ActiveEnum, DatabaseConnection};
use serde::{Deserialize, Serialize};

use entity::countries::Model as Country;
use entity::houses::Model as House;

use crate::AppState;
use crate::helpers::dto_builder_helpers::{country_helper, house_helper};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddPropertyPageDTO {
    id: String
}

#[derive(Debug, Clone)]
struct DTOBuilder {
    id: String,
}

#[get("/add-property")]
pub async fn get_add_property_page(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("get_add_property_page");
    let dbc = Arc::new(data.dbc.clone());
    let dto = build_dto(&dbc, |_builder| async { Ok(()) }).await?;
    Ok(HttpResponse::Ok().json(dto))
}

async fn build_dto<F, Fut>(dbc: &Arc<DatabaseConnection>, additional_processing: F) -> Result<AddPropertyPageDTO, Error>
    where
        F: FnOnce(&Arc<Mutex<DTOBuilder>>) -> Fut,
        Fut: Future<Output=Result<(), Error>>,
{
    let dto_builder = Arc::new(Mutex::new(DTOBuilder {
        id: String::new()
    }));

    additional_processing(&dto_builder).await?;

    let dto_builder_lock = dto_builder.lock().unwrap();
    Ok(to_home_page_dto(dto_builder_lock.clone()))
}

fn to_home_page_dto(dto_builder: DTOBuilder) -> AddPropertyPageDTO {
    AddPropertyPageDTO {
        id: String::from("id")
    }
}
