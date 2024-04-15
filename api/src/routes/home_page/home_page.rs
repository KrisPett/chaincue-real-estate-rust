use std::future::Future;
use std::io::Error;

use actix_web::{get, HttpResponse, web};
use sea_orm::{ActiveEnum, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use entity::countries::Entity as Countries;
use entity::countries::Model as Country;
use entity::houses::Entity as Houses;
use entity::houses::Model as House;

use crate::AppState;
use crate::middlewares::errors::CustomErrors;

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

struct DTOBuilder {
    countries: Vec<entity::countries::Model>,
    houses: Vec<entity::houses::Model>,
}

#[get("/home")]
pub async fn get_hey(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    log::info!("home");
   let d = &data.dbc.clone();
    let dto = build_dto(d, |dto_builder| {
        let future = async move {
            let countries = Countries::find()
                .all(d)
                .await
                .map_err(|err| Error::from(CustomErrors::DatabaseError(err)));
            // dto_builder.countries = countries.unwrap_or_default();
        };
        future
    }).await?;

    Ok(HttpResponse::Ok().json(dto))
}

async fn build_dto<F, Fut>(dbc: &DatabaseConnection, additional_processing: F) -> Result<HomePageDTO, Error>
    where
        F: FnOnce(&mut DTOBuilder) -> Fut,
        Fut: Future<Output=()>,
{

    let mut dto_builder = DTOBuilder {
        countries: Vec::new(),
        houses: Vec::new(),
    };
    additional_processing(&mut dto_builder).await;

    let countries = Countries::find()
        .all(dbc)
        .await
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;

    let houses = Houses::find()
        .all(dbc)
        .await
        .map_err(|err| Error::from(CustomErrors::DatabaseError(err)))?;

    dto_builder.countries = countries;
    dto_builder.houses = houses;

    Ok(to_home_page_dto(dto_builder))
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
