#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use crate::routes;
    use dotenv::dotenv;

    use crate::db::maria_lib::DataBase;
    use crate::db::meteo::meteo_sky::MeteorologicalSky;

    // #[actix_web::test]
    #[test]
    fn get_sky_data() {
      let mut db = DataBase::init();
      let lat: f64 = 34.7973052;
      let lon: f64 = 128.4642589;
      let obj = MeteorologicalSky::init(&db, lat, lon).await;

      println!("{:#?}", obj);

    }


  }