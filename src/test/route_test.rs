#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use mysql::prelude::*;
    use mysql::*;
    use serde::{Deserialize, Serialize};

    use crate::db::maria_lib::DataBase;
    use crate::db::model::{Buoy, Group};
    use crate::routes;
    use dotenv::dotenv;

    #[actix_web::test]
    //#[test]
    async fn main_group_test() {
        dotenv().ok();

        let mut app = test::init_service(App::new().service(routes::main_router::group)).await;

        let resp = test::TestRequest::get()
            .uri("/main/group")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn main_data_test() {
        dotenv().ok();

        let mut app =
            test::init_service(App::new().service(routes::main_router::get_main_data)).await;

        let resp = test::TestRequest::get()
            .uri("/main?location=tongyeong")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }
}
