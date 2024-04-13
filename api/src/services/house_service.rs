use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, DbErr, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use actix_web::{
    error, get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result, error::InternalError
};
use actix_web::http::StatusCode;
use entity::countries;
use entity::countries::Model;
use entity::sea_orm_active_enums::CountryName;
use crate::AppState;
use crate::errors::errors::CustomError;

#[derive(Serialize, Deserialize)]
struct CountryResponse {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    country_name: CountryName,
}

pub async fn insert(data: web::Data<AppState>) -> Result<Model, Error> {
    let dbc = &data.dbc;
    let country = countries::ActiveModel {
        // id: Set(String::from(Uuid::new_v4())),
        id: Set("5447aa63-58cc-4d83-ad1d-168e6a6095b3".parse()?),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        country_name: Set(Some(CountryName::Spain)),
    };

    match country.insert(dbc).await {
        Ok(model) => Ok(model),
        // Err(err) => Err(InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR).into()),
        Err(err) => Err(Error::from(CustomError::DatabaseError(err))),
    }
}
