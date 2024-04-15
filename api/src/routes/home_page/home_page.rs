use actix_web::{Error, get, HttpResponse, web};
use sea_orm::{ActiveEnum, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use entity::countries::Entity as Country;
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
    let dto = build_dto(&data.dbc).await?;
    Ok(HttpResponse::Ok().json(dto))
}

async fn build_dto(dbc: &DatabaseConnection) -> Result<HomePageDTO, Error> {
    let mut dto_builder = DTOBuilder {
        countries: Vec::new(),
        houses: Vec::new(),
    };

   let result= Country::find()
       .all(dbc)
       .await
       .map_err(|err| std::io::Error::from(CustomErrors::DatabaseError(err)))?;

    dto_builder.countries = result;

    Ok(to_home_page_dto(dto_builder))
}

fn to_home_page_dto(dto_builder: DTOBuilder) -> HomePageDTO {
    HomePageDTO {
        countries: dto_builder.countries.into_iter().map(to_country_dto).collect(),
        houses: dto_builder.houses.into_iter().map(to_house_dto).collect(),
    }
}

fn to_country_dto(country: entity::countries::Model) -> CountryDTO {
    CountryDTO {
        name: country.country_name.to_value()
    }
}

fn to_house_dto(house: entity::houses::Model) -> HouseDTO {
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
