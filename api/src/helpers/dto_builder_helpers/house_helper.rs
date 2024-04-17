use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use crate::services::house_service::find_all;
use entity::houses::Model as House;

pub fn update_dto_builder_with_houses<'a, B, F>(
    dbc: Arc<DatabaseConnection>,
    set_houses: F,
) -> impl FnOnce(&'a mut B) -> Pin<Box<dyn Future<Output=()> + Send + 'a>> + 'a
    where
        B: 'a + Send,
        F: Fn(&'a mut B, &Vec<House>) + Send + 'static,
{
    move |dto_builder: &'a mut B| {
        Box::pin(async move {
            let houses = find_all(&*dbc).await.unwrap_or_else(|_| Vec::new());
            set_houses(dto_builder, &houses);
        })
    }
}
