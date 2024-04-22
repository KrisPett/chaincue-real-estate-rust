use actix_web::{middleware, web, App, HttpServer};
use sea_orm::DatabaseConnection;

use configs::connect_db;
use configs::init_data::init_data;
use routes::home_page;
use routes::houses_page;
use routes::house_page;
use routes::account_page;
use routes::add_property_page;

mod configs;
mod helpers;
mod middlewares;
mod routes;
mod services;
mod utilities;

#[derive(Debug, Clone)]
struct AppState {
    dbc: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dbc = connect_db::connect_postgres().await?;
    init_data(&dbc).await?;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = AppState { dbc };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(houses_page::init_routes)
            .configure(home_page::init_routes)
            .configure(house_page::init_routes)
            .configure(account_page::init_routes)
            .configure(add_property_page::init_routes)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
