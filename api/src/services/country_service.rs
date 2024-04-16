use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::sync::Arc;

use sea_orm::{DatabaseConnection, EntityTrait};

use entity::countries::Model as Country;
use entity::prelude::Countries;

use crate::middlewares::errors::CustomErrors;

pub async fn find_all(db_conn: &DatabaseConnection) -> Result<Vec<Country>, Error>{
    let countries = Countries::find()
        .all(db_conn)
        .await
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;
    Ok(countries)
}

// pub fn update_dto_builder_with_countries<B>(dbc: &DatabaseConnection, set_countries: fn(&mut B, &Vec<Country>)) -> fn(&mut B) {
//     move |dto_builder: &mut B| {
//         let countries = find_all(dbc);
//         set_countries(dto_builder, &countries);
//     }
// }

pub fn update_dto_builder_with_countries<'a, B, F>(dbc: Arc<DatabaseConnection>, set_countries: F, ) -> impl FnOnce(&'a mut B) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> + 'a
    where
        B: 'a + Send,
        F: Fn(&'a mut B, &Vec<Country>) + Send + 'static,
{
    move |dto_builder: &'a mut B| {
        Box::pin(async move {
            let result = find_all(&*dbc).await;
            match result {
                Ok(countries) => set_countries(dto_builder, &countries),
                Err(e) => eprintln!("Failed to load countries: {}", e),
            }
        })
    }
}