use std::future::Future;
use std::io::Error;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use sea_orm::DatabaseConnection;

use entity::brokers::Model as Broker;
use entity::house_images::Model as HouseImage;
use entity::houses::Model as House;

use crate::services::house_service::{HouseOperations, HouseService};

pub fn update_dto_builder_with_houses<'a, B, F>(dbc: &'a Arc<DatabaseConnection>, set_houses: F) -> impl FnOnce(&'a Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where B: 'a + Send, F: Fn(&'a Arc<Mutex<B>>, Vec<House>) + Send + 'a, {
    move |dto_builder: &'a Arc<Mutex<B>>| {
        Box::pin(async move {
            let houses = HouseService.find_all_houses(dbc).await?;
            set_houses(dto_builder, houses);
            Ok(())
        })
    }
}

pub fn update_dto_builder_with_house_by_id<'a, B, F>(dbc: &'a Arc<DatabaseConnection>, id: &'a String, set_house: F) -> impl FnOnce(&'a Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where B: 'a + Send, F: Fn(&'a Arc<Mutex<B>>, Option<House>) + Send + 'a, {
    move |dto_builder: &'a Arc<Mutex<B>>| {
        Box::pin(async move {
            let house = HouseService.find_house_by_id(dbc, id).await?;
            set_house(dto_builder, house);
            Ok(())
        })
    }
}

pub fn update_dto_builder_with_broker_by_house_id<'a, B, F>(dbc: &'a Arc<DatabaseConnection>, house_id: &'a String, set_broker: F) -> impl FnOnce(&'a Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where B: 'a + Send, F: Fn(&'a Arc<Mutex<B>>, Option<Broker>) + Send + 'a, {
    move |dto_builder: &'a Arc<Mutex<B>>| {
        Box::pin(async move {
            let broker = HouseService.find_broker_by_house_id(dbc, &house_id).await?;
            set_broker(dto_builder, broker);
            Ok(())
        })
    }
}

pub fn update_dto_builder_with_house_images_by_house_id<'a, B, F>(dbc: &'a Arc<DatabaseConnection>, house_id: &'a String, set_house_images: F) -> impl FnOnce(&'a Arc<Mutex<B>>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + 'a>> + 'a
    where B: 'a + Send, F: Fn(&'a Arc<Mutex<B>>, Vec<HouseImage>) + Send + 'a, {
    move |dto_builder: &'a Arc<Mutex<B>>| {
        Box::pin(async move {
            let house_images = HouseService.find_house_images_by_house_id(dbc, &house_id).await?;
            set_house_images(dto_builder, house_images);
            Ok(())
        })
    }
}
