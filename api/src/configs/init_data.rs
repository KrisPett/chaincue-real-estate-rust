use std::io::Error;

use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use entity::countries;
use entity::sea_orm_active_enums::CountryName;

use crate::utilities::errors::CustomErrors;

pub async fn init_data(dbc: &DatabaseConnection) -> Result<(), Error> {
    dbc.execute(Statement::from_sql_and_values(DbBackend::Postgres, "DELETE FROM countries", []))
        .await
        .map_err(|err| CustomErrors::DatabaseError(err))?;

    let countries_to_insert = vec![
        countries::new_country(CountryName::Spain),
        countries::new_country(CountryName::Sweden),
    ];

    for country in countries_to_insert {
        country.insert(dbc).await.map_err(|err| CustomErrors::DatabaseError(err))?;
    }

    Ok(())
}
