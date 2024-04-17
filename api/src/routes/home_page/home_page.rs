use std::future::Future;
use std::io::Error;

use actix_web::{get, HttpResponse, web};
use sea_orm::{ActiveEnum, DatabaseConnection};
use serde::{Deserialize, Serialize};

use entity::countries::Model as Country;
use entity::houses::Model as House;

use crate::AppState;
use crate::helpers::dto_builder_helpers::{country_helper, house_helper};

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

#[derive(Serialize, Deserialize)]
struct HomePageDTO {
    countries: Vec<CountryDTO>,
    houses: Vec<HouseDTO>,
}

#[derive(Serialize, Deserialize)]
struct CountryDTO {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct HouseDTO {
    id: String,
    title: String,
    location: String,
    r#type: String,
    number_rooms: i64,
    beds: i64,
    dollar_price: String,
    crypto_price: String,
    src: String,
}

#[derive(Debug, Clone)]
struct DTOBuilder {
    countries: Vec<entity::countries::Model>,
    houses: Vec<entity::houses::Model>,
}

#[get("/home")]
pub async fn get_hey(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("home");
    let dbc = Arc::new(data.dbc.clone());
    let dto = build_dto(&dbc, |_builder| async { Ok(()) }).await?;
    Ok(HttpResponse::Ok().json(dto))
}

async fn build_dto<F, Fut>(dbc: &Arc<DatabaseConnection>, additional_processing: F) -> Result<HomePageDTO, Error>
    where
        F: FnOnce(Arc<DTOBuilder>) -> Fut,
        Fut: Future<Output=Result<(), Error>>,
{
    let mut dto_builder = Arc::new(Mutex::new(DTOBuilder {
        countries: Vec::new(),
        houses: Vec::new(),
    }));

    additional_processing(dto_builder.clone()).await?;

    country_helper::update_dto_builder_with_countries(dbc, |dto_builder: &mut DTOBuilder, countries| {
        println!("update_dto_builder_with_countries");
        dto_builder.countries = countries;
    })(&mut dto_builder).await?;

    house_helper::update_dto_builder_with_houses(dbc, |dto_builder: &mut DTOBuilder, houses| {
        println!("update_dto_builder_with_houses");
        dto_builder.houses = houses;
    })(&mut dto_builder).await?;
    let dto_builder_ref = dto_builder.lock().unwrap();
    Ok(to_home_page_dto(dto_builder_ref))
}

// async fn build_dto<F, Fut>(dbc: &Arc<DatabaseConnection>, additional_processing: F) -> Result<HomePageDTO, Error>
//     where
//         F: FnOnce(DTOBuilder) -> Fut,
//         Fut: Future<Output=Result<(), Error>>,
// {
//     // let mut dto_builder = Mutex::new(DTOBuilder {
//     //     countries: Vec::new(),
//     //     houses: Vec::new(),
//     // });
//
//     let dto_builder = Arc::new(Mutex::new(DTOBuilder {
//         countries: Vec::new(),
//         houses: Vec::new(),
//     }));
//
//     // let h1: task::JoinHandle<Result<(), Error>> = task::spawn(async move {
//     //     country_helper::update_dto_builder_with_countries(dbc, |dto_builder: &mut DTOBuilder, countries| {
//     //         dto_builder.countries = countries;
//     //     })(&mut dto_builder).await?;
//     //     Ok(())
//     // });
//
//     // h1.await??;
//
//     let dto_builder = dto_builder.lock().await;
//     Ok(to_home_page_dto(dto_builder.clone()))
// }

fn to_home_page_dto(dto_builder: DTOBuilder) -> HomePageDTO {
    HomePageDTO {
        countries: dto_builder.countries.into_iter().map(to_country_dto).collect(),
        houses: dto_builder.houses.into_iter().map(to_house_dto).collect(),
    }
}

fn to_country_dto(country: Country) -> CountryDTO {
    CountryDTO {
        name: country.country_name.to_value()
    }
}

fn to_house_dto(house: House) -> HouseDTO {
    HouseDTO {
        id: house.id,
        title: house.title.unwrap_or(String::new()),
        location: house.location.unwrap_or(String::new()),
        r#type: house.house_types.to_value(),
        number_rooms: house.number_rooms.unwrap_or(0),
        beds: house.beds.unwrap_or(0),
        dollar_price: "".to_string(),
        crypto_price: String::from("₿32.346"),
        src: house.src.unwrap_or(String::new()),
    }
}
