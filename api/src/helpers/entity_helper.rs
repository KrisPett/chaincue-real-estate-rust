use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use entity::sea_orm_active_enums::{CountryName, HouseTypes};

pub fn new_country(country_name: CountryName) -> entity::countries::ActiveModel {
    entity::countries::ActiveModel {
        id: Set(String::from(Uuid::new_v4())),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        country_name: Set(country_name),
    }
}

pub fn new_broker(name: String) -> entity::brokers::ActiveModel {
    entity::brokers::ActiveModel {
        id: Set(String::from(Uuid::new_v4())),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        name: Set(name),
        phone_number: Set(Some("".parse().unwrap())),
        email: Set(Some("".parse().unwrap())),
        house_id: Set(None),
    }
}

pub fn new_house(location: String, description: String, country: String, src: String, city: String, price: i64, house_types: HouseTypes) -> entity::houses::ActiveModel {
    entity::houses::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        title: Set(None),
        description: Set(Option::from(description)),
        location: Set(Option::from(location)),
        country: Set(Option::from(country)),
        city: Set(Option::from(city)),
        number_rooms: Set(None),
        beds: Set(None),
        price: Set(Option::from(price)),
        src: Set(Option::from(src)),
        sold: Set(None),
        house_types: Set(house_types),
    }
}

pub fn new_house_image(url: String, house_id: String) -> entity::house_images::ActiveModel {
    entity::house_images::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        url: Set(url),
        house_id: Set(house_id),
    }
}

