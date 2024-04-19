use std::future::Future;
use std::io::Error;
use std::sync::{Arc, Mutex, MutexGuard};

use actix_web::{get, HttpResponse, web};
use sea_orm::{ActiveEnum, DatabaseConnection};
use serde::{Deserialize, Serialize};

use entity::countries::Model as Country;
use entity::houses::Model as House;

use crate::AppState;
use crate::helpers::dto_builder_helpers::{country_helper, house_helper};

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
pub async fn get_home_page(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("home");
    let dbc = Arc::new(data.dbc.clone());
    let dto = build_dto(&dbc, |_builder| async { Ok(()) }).await?;
    Ok(HttpResponse::Ok().json(dto))
}

async fn build_dto<F, Fut>(dbc: &Arc<DatabaseConnection>, additional_processing: F) -> Result<HomePageDTO, Error>
    where
        F: FnOnce(&Arc<Mutex<DTOBuilder>>) -> Fut,
        Fut: Future<Output=Result<(), Error>>,
{
    let dto_builder = Arc::new(Mutex::new(DTOBuilder {
        countries: Vec::new(),
        houses: Vec::new(),
    }));

    additional_processing(&dto_builder).await?;

    let dto_builder_clone_for_countries = Arc::clone(&dto_builder);
    let dbc_clone_for_countries = Arc::clone(&dbc);

    let country_task = tokio::spawn(async move {
        country_helper::update_dto_builder_with_countries(&dbc_clone_for_countries, |dto_builder_mutex, countries| {
            println!("country_task");
            let mut dto_builder: MutexGuard<DTOBuilder> = dto_builder_mutex.lock().unwrap();
            dto_builder.countries = countries;
        })(&dto_builder_clone_for_countries).await
    });

    let dto_builder_clone_for_houses = Arc::clone(&dto_builder);
    let dbc_clone_for_houses = Arc::clone(&dbc);
    let house_task = tokio::spawn(async move {
        house_helper::update_dto_builder_with_houses(&dbc_clone_for_houses, |dto_builder_mutex, houses| {
            println!("house_task");
            let mut dto_builder: MutexGuard<DTOBuilder> = dto_builder_mutex.lock().unwrap();
            dto_builder.houses.sort_by(move |a, b| b.created_at.cmp(&a.created_at));
            dto_builder.houses = houses.into_iter().take(6).collect();
        })(&dto_builder_clone_for_houses).await
    });

    country_task.await??;
    house_task.await??;

    let dto_builder_lock = dto_builder.lock().unwrap();
    Ok(to_home_page_dto(dto_builder_lock.clone()))
}

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
