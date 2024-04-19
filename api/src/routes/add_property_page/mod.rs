use actix_web::web;
use crate::routes::account_page::account_page;

pub mod add_property_page;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(account_page::get_account_page);
}
