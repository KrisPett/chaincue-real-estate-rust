pub enum CountryName {
    SWEDEN,
    SPAIN
}

#[derive(Debug)]
pub struct Country {
    pub id: String,
    pub name: CountryName
}

impl Country {
    pub fn new(country_name: CountryName) -> Country {
        Country {
            id: "1",
            name: country_name
        }
    }
}
