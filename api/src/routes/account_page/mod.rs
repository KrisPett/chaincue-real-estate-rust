use actix_web::web;

pub mod account_page;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(account_page::get_account_page);
}
