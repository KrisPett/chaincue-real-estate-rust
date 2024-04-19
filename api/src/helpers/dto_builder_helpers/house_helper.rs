use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use sea_orm::DatabaseConnection;

use entity::houses::Model as House;

use crate::services::house_service::{find_all, find_by_id};

pub fn update_dto_builder_with_houses<'a, B, F>(
    dbc: &'a Arc<DatabaseConnection>,
    set_houses: F,
) -> impl FnOnce(&'a Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where
        B: 'a + Send,
        F: Fn(&'a Arc<Mutex<B>>, Vec<House>) + Send + 'a,
{
    move |dto_builder: &'a Arc<Mutex<B>>| {
        Box::pin(async move {
            let houses = find_all(dbc).await?;
            set_houses(dto_builder, houses);
            Ok(())
        })
    }
}

pub fn update_dto_builder_with_house_by_id<'a, B, F>(
    dbc: &'a Arc<DatabaseConnection>,
    id: String,
    set_house: F,
) -> impl FnOnce(&'a Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where
        B: 'a + Send,
        F: Fn(&'a Arc<Mutex<B>>, House) + Send + 'a,
{
    move |dto_builder: &'a Arc<Mutex<B>>| {
        Box::pin(async move {
            let houses = find_by_id(dbc, id).await?;
            set_house(dto_builder, houses);
            Ok(())
        })
    }
}