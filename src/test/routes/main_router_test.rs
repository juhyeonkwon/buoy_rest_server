#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use crate::custom_middleware;
    use crate::routes;
    use dotenv::dotenv;
    use std::env;

    #[actix_web::test]
    //#[test]
    async fn main_group_data_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");

        let mut app = test::init_service(
            App::new().service(
                web::scope("/main")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::main_router::get_location_data),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/main/data?latitude=35.1513466&longitude=128.1001125")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn main_group_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");

        let mut app = test::init_service(
            App::new().service(
                web::scope("/main")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::main_router::group),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .append_header(("Authorization", token))
            .uri("/main/group")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn main_group_total_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");

        let mut app = test::init_service(
            App::new().service(
                web::scope("/main")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::main_router::group_total),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .append_header(("Authorization", token))
            .uri("/main/group/total")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn main_warn_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");

        let mut app = test::init_service(
            App::new().service(
                web::scope("/main")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::main_router::get_main_warn),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .append_header(("Authorization", token))
            .uri("/main/warn")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }
}
