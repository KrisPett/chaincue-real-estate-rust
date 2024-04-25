#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use actix_web::{App, test, web, web::Data};

    use crate::AppState;
    use crate::configs::connect_db;
    use crate::configs::init_data::init_data;
    use crate::routes::house_page::house_page;

    use super::*;

    #[actix_web::test]
    async fn test_home_page() {
        // Given
        let dbc = connect_db::connect_postgres().await.unwrap();
        // init_data(&dbc).await.unwrap();
        let state = AppState { dbc };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .service(house_page::get_house_page)
        )
            .await;
        let path = String::from("/house/27248d2b-29ad-44b1-a99d-5e70b3ca829a");
        let req = test::TestRequest::get().uri(&path).to_request();

        // When
        let resp = test::call_service(&app, req).await;

        // Then
        let body = test::read_body(resp).await;
        let json_body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let house = &json_body;
        println!("{:?}", house);
    }
}
