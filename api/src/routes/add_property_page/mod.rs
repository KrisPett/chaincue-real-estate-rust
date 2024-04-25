use actix_web::web;

pub mod add_property_page;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_property_page::get_add_property_page);
}
