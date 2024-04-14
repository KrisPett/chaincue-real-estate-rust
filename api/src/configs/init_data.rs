use std::convert::From;
use std::io::Error;

use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm::ActiveValue::Set;

use entity::brokers::ActiveModel;
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
    let broker_clone = broker.clone();
    broker.insert(dbc).await.map_err(|err| CustomErrors::DatabaseError(err))?;

    // house
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_1), HouseTypes::Villa, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969381, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_2), HouseTypes::Villa, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969382, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_3), HouseTypes::Villa, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969383, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_4), HouseTypes::Villa, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969384, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_5), HouseTypes::VacationHome, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969385, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_6), HouseTypes::VacationHome, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969386, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_1), HouseTypes::VacationHome, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969387, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_2), HouseTypes::EstatesAndFarms, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969388, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_3), HouseTypes::EstatesAndFarms, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969389, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_4), HouseTypes::EstatesAndFarms, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969311, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_5), HouseTypes::Land, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969321, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_6), HouseTypes::Land, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969331, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_1), HouseTypes::Land, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969341, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_2), HouseTypes::OtherHouses, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969351, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_3), HouseTypes::OtherHouses, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969361, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_4), HouseTypes::OtherHouses, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969371, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_5), HouseTypes::Townhouse, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969381, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_6), HouseTypes::Townhouse, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969181, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_1), HouseTypes::Townhouse, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969281, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_2), HouseTypes::Townhouse, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969381, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_3), HouseTypes::Condominium, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969481, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_4), HouseTypes::Condominium, String::from("SWEDEN"), String::from("Sweden, Stockholm"), String::from("Stockholm"), 969581, ).await?;
    create_house(dbc, broker_clone.clone(), String::from(aws_s3_urls::URL_FRONT_IMAGE_5), HouseTypes::Condominium, String::from("SPAIN"), String::from("Spain, Málaga"), String::from("Stockholm"), 969681, ).await?;

    Ok(())
}

async fn create_house(dbc: &DatabaseConnection, mut broker: ActiveModel, src: String, house_type: HouseTypes, country: String, location: String, city: String, price: i64) -> Result<(), Error> {
    let description = "Welcome to this bright and well-planned four-bedroom apartment with a balcony in a private location and a view of greenery! The residence features well-organized rooms and substantial windows in three different directions, providing a delightful infusion of natural light throughout the entire apartment. You'll find a spacious living room with comfortable seating areas and access to the pleasant balcony, offering sunny exposure and a lovely view of the green surroundings. Additionally, the apartment boasts a spacious kitchen with room for a dining area for the whole family, and here too, you can enjoy a pleasant view of the green area outside.\n\nThis well-planned apartment includes three good-sized bedrooms. Conveniently, for larger families, it offers both a fully tiled bathroom with a washing machine and a guest WC. Ample storage options are available through closets and a walk-in closet.\n\nYou are warmly welcome to visit!";

    let house = entity_helper::new_house(
        String::from(location),
        String::from(description),
        String::from(country),
        String::from(src),
        String::from(city),
        price,
        house_type,
    );

    let cloned_house1 = house.clone();
    let cloned_house2 = house.clone();
    house.insert(dbc).await.map_err(|err| CustomErrors::DatabaseError(err))?;

    broker.house_id = Set(Some(cloned_house1.id.unwrap()));

    let result = broker.update(dbc).await;
    let _ = result.map_err(|err| CustomErrors::DatabaseError(err))?;

    let image_urls = [
        aws_s3_urls::URL_1,
        aws_s3_urls::URL_2,
        aws_s3_urls::URL_3,
        aws_s3_urls::URL_4,
        aws_s3_urls::URL_5,
        aws_s3_urls::URL_6,
    ];

    for url in image_urls.iter() {
        let house_image = entity_helper::new_house_image(url.to_string(), cloned_house2.clone().id.unwrap());
        house_image.insert(dbc).await.map_err(|err| CustomErrors::DatabaseError(err))?;
    }

    Ok(())
}
