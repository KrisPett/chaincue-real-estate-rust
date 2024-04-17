use std::future::Future;
use std::pin::Pin;

use sea_orm::DatabaseConnection;

use entity::countries::Model as Country;

use crate::services::country_service::find_all;

pub fn update_dto_builder_with_countries<'a, B, F>(
    dbc: &'a DatabaseConnection,
    set_countries: F,
) -> impl FnOnce(&'a mut B) -> Pin<Box<dyn Future<Output=()> + Send + 'a>> + 'a
    where
        B: 'a + Send,
        F: Fn(&'a mut B, &Vec<Country>) + Send + 'static,
{
    move |dto_builder: &'a mut B| {
        Box::pin(async move {
            let countries = find_all(dbc).await.unwrap_or_else(|_| Vec::new());
            set_countries(dto_builder, &countries);
        })
    }
}
