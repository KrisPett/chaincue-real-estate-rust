use actix_web::web;

pub mod home_page;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home_page::get_home_page);
}
