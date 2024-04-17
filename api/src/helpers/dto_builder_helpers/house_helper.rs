use std::future::Future;
use std::io::Error;
use std::pin::Pin;

use sea_orm::DatabaseConnection;

use entity::houses::Model as House;

use crate::services::house_service::find_all;

pub fn update_dto_builder_with_houses<'a, B, F>(
    dbc: &'a DatabaseConnection,
    set_houses: F,
) -> impl FnOnce(&'a mut B) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where
        B: 'a + Send,
        F: Fn(&'a mut B, &Vec<House>) + Send + 'static,
{
    move |dto_builder: &'a mut B| {
        Box::pin(async move {
            let houses = find_all(dbc).await?;
            set_houses(dto_builder, &houses);
            Ok(())
        })
    }
}