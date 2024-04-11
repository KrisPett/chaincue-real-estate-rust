use actix_web::{get, HttpResponse, Responder, web};
use sea_orm::ActiveModelTrait;

use crate::AppState;
use crate::services::house_service;

#[get("/houses")]
pub async fn get_hey(data: web::Data<AppState>) -> impl Responder {
    let dto = house_service::insert(data).await?;

    Ok(dto).expect("TODO: panic message");

}

#[get("/houses2")]
pub async fn get_bye(data: web::Data<AppState>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body("houses2")
}
