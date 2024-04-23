#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use actix_web::{App, test, web, web::Data};

    use crate::AppState;
    use crate::configs::connect_db;
    use crate::configs::init_data::init_data;
    use crate::routes::home_page::home_page;

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
                .service(home_page::get_home_page)
        )
            .await;
        let req = test::TestRequest::get().uri("/home").to_request();

        // When
        let resp = test::call_service(&app, req).await;

        // Then
        assert!(resp.status().is_success(), "Expected success status code");

        let body = test::read_body(resp).await;
        let json_body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let house = &json_body["houses"][0];
        println!("{:?}", house);

        assert_eq!(json_body["countries"][0]["name"], "SPAIN");
        assert_eq!(json_body["countries"][1]["name"], "SWEDEN");
        assert_eq!(json_body["houses"][0]["location"], "Sweden, Stockholm");
    }
}
