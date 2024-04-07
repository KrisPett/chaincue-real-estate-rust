use actix_web::{App, HttpServer, middleware};

use configs::connect_db;
use routes::home_page;
use routes::houses_page;

mod routes;
mod configs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    connect_db::connect_postgres().await?;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(houses_page::init_routes)
            .configure(home_page::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
