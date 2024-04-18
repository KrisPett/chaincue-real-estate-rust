use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use sea_orm::DatabaseConnection;

use entity::houses::Model as House;

use crate::services::house_service::find_all;

pub fn update_dto_builder_with_houses<'a, B, F>(
    dbc: &'a Arc<DatabaseConnection>,
    set_houses: F,
) -> impl FnOnce(&'a mut Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where
        B: 'a + Send,
        F: Fn(&'a mut Arc<Mutex<B>>, Vec<House>) + Send + 'static,
{
    move |dto_builder: &'a mut Arc<Mutex<B>>| {
        Box::pin(async move {
            let houses = find_all(dbc).await?;
            set_houses(dto_builder, houses);
            Ok(())
        })
    }
}