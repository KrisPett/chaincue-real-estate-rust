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
        let path = String::from("/house/9f33bc8f-945e-44ac-9ce3-11ecc0d93058");
        let req = test::TestRequest::get().uri(&path).to_request();

        // When
        let resp = test::call_service(&app, req).await;

        // Then
        assert!(resp.status().is_success(), "Expected success status code");
    }
}
