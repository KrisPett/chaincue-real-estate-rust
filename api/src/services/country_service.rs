use std::io::Error;
use std::sync::Arc;

use sea_orm::{DatabaseConnection, EntityTrait};

use entity::countries::Model as Country;
use entity::prelude::Countries;

use crate::middlewares::errors::CustomErrors;

pub async fn find_all(db_conn: &DatabaseConnection) -> Result<Vec<Country>, Error> {
    let countries = Countries::find()
        .all(db_conn)
        .await
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;
    Ok(countries)
}
