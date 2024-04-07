use actix_web::web;

pub mod houses_page;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(houses_page::get_hey);
    cfg.service(houses_page::get_bye);
}
