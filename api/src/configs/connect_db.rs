use std::{env, io};
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use migration::MigratorTrait;

use crate::middlewares::errors::CustomErrors;

pub async fn connect_postgres() -> Result<DatabaseConnection, io::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public");

    let dbc = Database::connect(opt)
        .await
        .map_err(|err| CustomErrors::DatabaseError(err))?;

    migration::Migrator::up(&dbc, None).await.map_err(|err| {
        eprintln!("Migration error: {:?}", err);
        io::Error::new(io::ErrorKind::Other, "Migration error")
    })?;

    Ok(dbc)
}
