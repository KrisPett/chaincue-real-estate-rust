use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};


use sea_orm::DatabaseConnection;

use entity::countries::Model as Country;

use crate::services::country_service::find_all;

// pub fn update_dto_builder_with_countries<'a, B, F>(
//     dbc: &'a Arc<DatabaseConnection>,
//     set_countries: F,
// ) -> impl FnOnce(&'a mut Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
//     where
//         B: 'a + Send,
//         F: Fn(&'a mut B, Vec<Country>) + Send + 'static,
// {
//     move |dto_builder: &'a mut B| {
//         Box::pin(async move {
//             let countries = find_all(dbc).await?;
//             set_countries(dto_builder, countries);
//             Ok(())
//         })
//     }
// }

pub fn update_dto_builder_with_countries<'a, B, F>(
    dbc: &'a Arc<DatabaseConnection>,
    set_countries: F,
) -> impl FnOnce(&'a mut Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where
        B: 'a + Send,
        F: Fn(&'a mut Arc<Mutex<B>>, Vec<Country>) + Send + 'static,
{
    move |dto_builder: &'a mut Arc<Mutex<B>>| {
        Box::pin(async move {
            let countries = find_all(dbc).await?;
            set_countries(dto_builder, countries);
            Ok(())
        })
    }
}
