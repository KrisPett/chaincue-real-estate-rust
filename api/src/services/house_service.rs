use actix_web::{HttpResponse, Responder, web};
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use entity::countries;
use serde::{Deserialize, Serialize};
use entity::sea_orm_active_enums::CountryName;

use crate::AppState;

#[derive(Serialize, Deserialize)]
struct CountryResponse {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    country_name: CountryName,
}

pub async fn insert(data: web::Data<AppState>) -> impl Responder {
    let dbc = &data.dbc;
    let country = countries::ActiveModel {
        id: Set(String::from(Uuid::new_v4())),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        country_name: Set(Some(CountryName::Spain)),
    };

    match country.insert(dbc).await {
        Ok(model) => {
            let response = CountryResponse {
                id: model.id.unwrap_or_default(),
                created_at: model.created_at.unwrap_or(Utc::now()),
                updated_at: model.updated_at.unwrap_or(Utc::now()),
                country_name: CountryName::Spain

            };
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            eprintln!("Failed to insert country: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to insert country")
        }
    }

}
