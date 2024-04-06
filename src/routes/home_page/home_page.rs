use actix_web::{get, HttpResponse, Responder};

#[get("/home")]
pub async fn get_hey() -> impl Responder {
    HttpResponse::Ok().body("home!")
}
