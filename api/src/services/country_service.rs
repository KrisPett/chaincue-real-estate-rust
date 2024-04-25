use std::io::Error;

use sea_orm::{DatabaseConnection, EntityTrait};

use entity::countries::Model as CountryModel;
use entity::prelude::Countries;

use crate::middlewares::errors::CustomErrors;

pub trait CountryServiceI {
    async fn find_all(&self, dbc: &DatabaseConnection) -> Result<Vec<CountryModel>, Error>;
}

pub struct CountryService;

impl CountryServiceI for CountryService {
    async fn find_all(&self, dbc: &DatabaseConnection) -> Result<Vec<CountryModel>, Error> {
        let countries = Countries::find()
            .all(dbc)
            .await
            .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;
        Ok(countries)
    }
}
