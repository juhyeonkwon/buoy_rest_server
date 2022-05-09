#[cfg(test)]
mod tests {
    use actix_web::{test, App, web};

    use crate::routes;
    use dotenv::dotenv;
    use std::env;
    use serde_json::{Value, json};

    use rand::prelude::*;


    #[actix_web::test]
    //#[test]
    async fn user_login_test() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(web::scope("/user")
              .service(routes::auth_router::login)),
        )
        .await;

        let json = json!({
          "email" : "test@test.com",
          "password": "test"
        });

        let resp = test::TestRequest::post()
            .uri("/user/login")
            .set_json(json)
            .send_request(&mut app)
            .await;      

        assert!(resp.status().is_success());
    }


    #[actix_web::test]
    //#[test]
    async fn user_login_regist_test() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(web::scope("/user")
              .service(routes::auth_router::register)),
        )
        .await;

        let mut rng = rand::thread_rng();
        let ran : i32 = rng.gen_range(100..9999);

        let json = json!({
          "email" : String::from("test") + &ran.to_string() + "@test.com",
          "password": "test",
          "name": String::from("test") + &ran.to_string()
        });

        let resp = test::TestRequest::post()
            .uri("/user/register")
            .set_json(json)
            .send_request(&mut app)
            .await;      

        assert!(resp.status().is_success());
    }


}