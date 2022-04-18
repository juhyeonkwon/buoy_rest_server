use crate::db::maria_lib::DataBase;
use crate::db::redis_lib::connect_redis;

use crate::db::model::{Buoy, Group};

use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::routes::functions::main_data::get_meteo_data;
use crate::routes::functions::main_data::get_near_obs_data;
use crate::routes::functions::main_data::get_near_tide_data;
use crate::routes::functions::main_data::get_near_wave_data;

#[derive(Serialize, Deserialize)]
pub struct RealLocation {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Serialize)]
struct Obj {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub location: String,
}

#[get("/main/data")]
pub async fn get_location_data(query: web::Query<RealLocation>) -> impl Responder {
    let mut db = DataBase::init();
    let mut conn = connect_redis();

    let lat: f64 = query.latitude.parse().expect("Error!");
    let lon: f64 = query.longitude.parse().expect("Error!");

    let obs_val: serde_json::Value = get_near_obs_data(&mut db, &mut conn, &lat, &lon);
    let wave_val: serde_json::Value = get_near_wave_data(&mut db, &mut conn, &lat, &lon);
    let tide_val: serde_json::Value = get_near_tide_data(&mut db, &mut conn, &lat, &lon);
    let meteo_val: serde_json::Value = get_meteo_data(&mut db, &lat, &lon);

    web::Json(json!({
        "obs_data" : obs_val,
        "tidal" : tide_val,
        "wave_hight" : wave_val,
        "meteo_val" : meteo_val
    }))
}

#[get("/main/region")]
pub async fn get_main_data_region(query: web::Query<Location>) -> impl Responder {
    let mut conn = connect_redis();

    let _key = String::from(&query.location) + "_main_data";

    let mut _val: String = String::from("");

    let _: () = match conn.get(&_key) {
        Ok(v) => _val = v,
        Err(_) => return web::Json(json!({"error" : true})),
    };

    let json_data: Value = serde_json::from_str(&_val).expect("Error!!");

    web::Json(json!(json_data))
}

#[get("/main/group")]
pub async fn group() -> impl Responder {
    let mut db = DataBase::init();

    let query = r"SELECT * FROM buoy_group";

    let row: Vec<Group> = db
        .conn
        .query_map(
            query,
            |(
                group_id,
                group_name,
                group_latitude,
                group_longitude,
                group_water_temp,
                group_salinity,
                group_height,
                group_weight,
            )| Group {
                group_id,
                group_name,
                group_latitude,
                group_longitude,
                group_water_temp,
                group_salinity,
                group_height,
                group_weight,
            },
        )
        .expect("select Error");

    web::Json(row)
}
