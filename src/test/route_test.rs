#[cfg(test)]
mod tests {
    use mysql::prelude::*;
    use mysql::*;
    use serde::{Deserialize, Serialize};
    use actix_web::{test, App};

    use crate::db::maria_lib::DataBase;
    use crate::db::model::{Buoy, Group};
    use dotenv::dotenv;
    use crate::routes;

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
}
