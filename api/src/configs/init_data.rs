use std::convert::From;
use std::io::Error;

use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use entity::sea_orm_active_enums::{CountryName, HouseTypes};

use crate::helpers::entity_helper;
use crate::middlewares::errors::CustomErrors;
use crate::utilities::aws_s3_urls;

pub async fn init_data(dbc: &DatabaseConnection) -> Result<(), Error> {
    dbc.execute(Statement::from_sql_and_values(DbBackend::Postgres, "DELETE FROM countries", [])).await.map_err(|err| CustomErrors::DatabaseError(err))?;
    dbc.execute(Statement::from_sql_and_values(DbBackend::Postgres, "DELETE FROM brokers", [])).await.map_err(|err| CustomErrors::DatabaseError(err))?;
    dbc.execute(Statement::from_sql_and_values(DbBackend::Postgres, "DELETE FROM houses", [])).await.map_err(|err| CustomErrors::DatabaseError(err))?;
    dbc.execute(Statement::from_sql_and_values(DbBackend::Postgres, "DELETE FROM house_images", [])).await.map_err(|err| CustomErrors::DatabaseError(err))?;

    // countries
    let countries_to_insert = vec![
        entity_helper::new_country(CountryName::Spain),
        entity_helper::new_country(CountryName::Sweden),
    ];
    for country in countries_to_insert {
        country.insert(dbc).await.map_err(|err| CustomErrors::DatabaseError(err))?;
    }
    //broker
    let broker = entity_helper::new_broker(String::from("John"));
    broker.insert(dbc).await.map_err(|err| CustomErrors::DatabaseError(err))?;
    // house
    create_house(dbc).await?;

    Ok(())
}

async fn create_house(dbc: &DatabaseConnection) -> Result<(), Error> {
    let description = "Welcome to this bright and well-planned four-bedroom apartment with a balcony in a private location and a view of greenery! The residence features well-organized rooms and substantial windows in three different directions, providing a delightful infusion of natural light throughout the entire apartment. You'll find a spacious living room with comfortable seating areas and access to the pleasant balcony, offering sunny exposure and a lovely view of the green surroundings. Additionally, the apartment boasts a spacious kitchen with room for a dining area for the whole family, and here too, you can enjoy a pleasant view of the green area outside.\n\nThis well-planned apartment includes three good-sized bedrooms. Conveniently, for larger families, it offers both a fully tiled bathroom with a washing machine and a guest WC. Ample storage options are available through closets and a walk-in closet.\n\nYou are warmly welcome to visit!";

    let house = entity_helper::new_house(
        String::from("Sweden, Stockholm"),
        String::from(description),
        String::from(aws_s3_urls::URL_FRONT_IMAGE_1),
        String::from("SWEDEN"),
        String::from("Stockholm"),
        969381,
        HouseTypes::Condominium,
    );

    let cloned_house = house.clone();
    house.insert(dbc).await.map_err(|err| CustomErrors::DatabaseError(err))?;

    let image_urls = [
        aws_s3_urls::URL_1,
        aws_s3_urls::URL_2,
        aws_s3_urls::URL_3,
        aws_s3_urls::URL_4,
        aws_s3_urls::URL_5,
        aws_s3_urls::URL_6,
    ];

    for url in image_urls.iter() {
        let house_image = entity_helper::new_house_image(url.to_string(), cloned_house.clone().id.unwrap());
        house_image.insert(dbc).await.map_err(|err| CustomErrors::DatabaseError(err))?;
    }

    Ok(())
}
