use actix_web::web;

pub mod house_page;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(house_page::get_house_page);
}
