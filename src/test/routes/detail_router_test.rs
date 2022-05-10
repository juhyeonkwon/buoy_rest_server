#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use crate::custom_middleware;
    use crate::routes;
    use crate::db;
    use dotenv::dotenv;
    use std::env;

    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[actix_web::test]
    //#[test]
    async fn detail_group_list_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                    .app_data(web::Data::new(pool.clone()))
                    .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::group_list),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/group/list")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_test() {
        dotenv().ok();
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");

        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")        
                    .app_data(web::Data::new(pool.clone()))
                    .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::group_detail),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/group?group_id=1")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_web_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                    .app_data(web::Data::new(pool.clone()))
                    .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::group_detail_web),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/group/web?group_id=1")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_history_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                    .app_data(web::Data::new(pool.clone()))
                    .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::group_history),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/group/history?group_id=1")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_buoy_spec_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                    .app_data(web::Data::new(pool.clone()))
                    .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::buoy_spec),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/buoy?model=buoy_1")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_buoy_list_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::buoy_group_list),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/buoy/list?group_id=1")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_buoy_history_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::buoy_detail),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/buoy/history?model=buoy_1")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_buoy_assigned_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::buoy_assigned),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/buoy/assigned")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_buoy_unassigned_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::buoy_unassigned),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/detail/buoy/unassigned")
            .append_header(("Authorization", token))
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[derive(Deserialize, Serialize)]
    struct Allocate {
        model: String,
        group_id: i32,
        line: i32,
    }

    #[derive(Deserialize, Serialize)]
    struct DeAllocate {
        model: String,
    }

    #[actix_web::test]
    //#[test]
    async fn detail_group_allocate_unallocate_test() {
        dotenv().ok();

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");
        let pool = db::maria_lib::DataBase::init().pool; 
        let redis_conn = db::redis_lib::get_client();
        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::buoy_allocate),
            ),
        )
        .await;

        let json: Allocate = Allocate {
            model: "buoy_101".to_owned(),
            group_id: 4,
            line: 1,
        };

        let resp = test::TestRequest::put()
            .uri("/detail/buoy/allocate")
            .append_header(("Authorization", token))
            .append_header(("Content-type", "application/json"))
            .set_json(json)
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());

        let mut app = test::init_service(
            App::new().service(
                web::scope("/detail")
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(redis_conn.clone()))
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::buoy_deallocate),
            ),
        )
        .await;

        let json: DeAllocate = DeAllocate {
            model: "buoy_101".to_owned(),
        };

        let token = String::from("Bearer ") + &env::var("TEST_KEY").expect("ENV not Found");

        let resp = test::TestRequest::put()
            .uri("/detail/buoy/deallocate")
            .append_header(("Authorization", token))
            .append_header(("Content-type", "application/json"))
            .set_json(json)
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }
}
