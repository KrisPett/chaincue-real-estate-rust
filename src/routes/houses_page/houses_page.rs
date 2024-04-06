use actix_web::{get, HttpResponse, Responder};

#[get("/houses")]
pub async fn get_hey() -> impl Responder {
    HttpResponse::Ok().body("houses!")
}

#[get("/houses2")]
pub async fn get_bye() -> impl Responder {
    HttpResponse::Ok().body("houses2")
}
