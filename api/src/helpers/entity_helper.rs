use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use entity::sea_orm_active_enums::CountryName;

pub fn new_country(country_name: CountryName) -> entity::countries::ActiveModel {
    entity::countries::ActiveModel {
        id: Set(String::from(Uuid::new_v4())),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        updated_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        country_name: Set(Some(country_name)),
    }
}

// pub fn new_broker(name: String) -> entity::brokers::ActiveModel {
//     entity::brokers::ActiveModel {
//         id: Set(String::from(Uuid::new_v4())),
//         created_at: Set(Option::from(DateTimeWithTimeZone::from(chrono::Utc::now()))),
//         updated_at: Set(Option::from(DateTimeWithTimeZone::from(chrono::Utc::now()))),
//         name: Set(Option::from(name)),
//         phone_number: Set(Some("".parse().unwrap())),
//         email: Set(Some("".parse().unwrap())),
//         house_id: Set(None),
//     }
// }
