#[cfg(test)]
mod tests {
    use actix_web::{test, App};

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
    async fn main_data_region_test() {
        dotenv().ok();

        let mut app =
            test::init_service(App::new().service(routes::main_router::get_main_data_region)).await;

        let resp = test::TestRequest::get()
            .uri("/main/region?location=tongyeong")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn main_data_location() {
        dotenv().ok();

        let mut app =
            test::init_service(App::new().service(routes::main_router::get_location_data)).await;

        let resp = test::TestRequest::get()
            .uri("/main/data?latitude=35.1513466&longitude=128.1001125")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn main_sky() {
        dotenv().ok();

        let mut app =
            test::init_service(App::new().service(routes::main_router::get_sky_data)).await;

        let resp = test::TestRequest::get()
            .uri("/main/data/sky?latitude=35.1513466&longitude=128.1001125")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn main_warn_test() {
        dotenv().ok();

        let mut app =
            test::init_service(App::new().service(routes::main_router::get_main_warn)).await;

        let resp = test::TestRequest::get()
            .uri("/main/warn")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    
    #[actix_web::test]
    //#[test]
    async fn detail_buoy_group_list() {
        dotenv().ok();

        let mut app =
            test::init_service(App::new().service(routes::detail_router::buoy_group_list)).await;

        let resp = test::TestRequest::get()
            .uri("/detail/buoy/list?group=A")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    //#[test]
    async fn group_history_list() {
        dotenv().ok();

        let mut app =
            test::init_service(App::new().service(routes::detail_router::group_history)).await;

        let resp = test::TestRequest::get()
            .uri("/detail/group/history?group=A")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }
}
