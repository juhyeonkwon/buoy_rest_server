#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use crate::custom_middleware;
    use crate::routes;
    use dotenv::dotenv;
    use std::env;

    #[actix_web::test]
    //#[test]
    async fn manage_user_list_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");

        let mut app = test::init_service(
            App::new().service(
                web::scope("/manage")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::manage_router::user_list),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/manage/user/list")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn manage_buoy_unassigned_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");

        let mut app = test::init_service(
            App::new().service(
                web::scope("/manage")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::manage_router::buoy_unassigned),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/manage/buoy/unassigned")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

}
