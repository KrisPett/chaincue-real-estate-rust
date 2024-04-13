use actix_web::{Error, get, HttpResponse, Responder, web};
use sea_orm::ActiveModelTrait;

use crate::AppState;
use crate::services::house_service;

#[get("/houses")]
pub async fn get_hey(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let dto = house_service::insert(data).await?;
    println!("ss");
    Ok(HttpResponse::Found()
        .finish())
}

#[get("/houses2")]
pub async fn get_bye(data: web::Data<AppState>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body("houses2")
}
