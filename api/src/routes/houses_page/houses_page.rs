use actix_web::{Error, get, HttpResponse, Responder, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use entity::sea_orm_active_enums::CountryName;

use crate::AppState;
use crate::services::house_service;

#[derive(Serialize, Deserialize)]
struct CountryResponse {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    country_name: CountryName,
}

#[get("/houses")]
pub async fn get_hey(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let model = house_service::insert(&data.dbc).await?;
    let country_response = CountryResponse {
        id: model.id,
        created_at: DateTime::from(model.created_at),
        updated_at: DateTime::from(model.updated_at),
        country_name: model.country_name,
    };
    Ok(HttpResponse::Ok().json(country_response))
}

#[get("/houses2")]
pub async fn get_bye(data: web::Data<AppState>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body("houses2")
}
