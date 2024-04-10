use actix_web::{web, App, HttpServer, middleware};
use sea_orm::DatabaseConnection;

use configs::connect_db;
use routes::home_page;
use routes::houses_page;

mod routes;
mod configs;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = connect_db::connect_postgres().await.unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(houses_page::init_routes)
            .configure(home_page::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
