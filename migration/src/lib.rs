pub use sea_orm_migration::prelude::*;

mod m20240409_171941_country;
mod m20240410_170204_house;
mod m20240410_170304_house_image;
mod m20240410_180112_broker;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240409_171941_country::Migration),
            Box::new(m20240410_170204_house::Migration),
            Box::new(m20240410_170304_house_image::Migration),
            Box::new(m20240410_180112_broker::Migration),
        ]
    }
}
