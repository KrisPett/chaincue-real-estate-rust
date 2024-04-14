use std::io::Error;

use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use entity::countries;
use entity::sea_orm_active_enums::CountryName;

use crate::utilities::errors::CustomErrors;

pub async fn init_data(dbc: &DatabaseConnection) -> Result<(), Error> {
    let clean_up_db = Statement::from_sql_and_values(DbBackend::Postgres, "DELETE FROM countries", []);

     match dbc.execute(clean_up_db).await {
        Ok(_) => println!("Database cleanup successful"),
        Err(err) => return Err(Error::from(CustomErrors::DatabaseError(err))),
    };


    let country1 = countries::new_country(CountryName::Spain);
    let country2 = countries::new_country(CountryName::Sweden);

    let _ = match country1.insert(dbc).await {
        Ok(model) => Ok(model),
        Err(err) => Err(CustomErrors::DatabaseError(err)),
    };
    let _ = match country2.insert(dbc).await {
        Ok(model) => Ok(model),
        Err(err) => Err(CustomErrors::DatabaseError(err)),
    };

    Ok(())
}
