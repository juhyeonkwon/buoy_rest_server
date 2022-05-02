use crate::db::maria_lib::DataBase;
use crate::db::redis_lib::connect_redis;

use crate::db::model::main_model::{RealLocation, Location, Total, MainGroupList};
use actix_web::{get, /*post,*/ web, /*HttpResponse,*/ Responder};
use mysql::prelude::*;
use mysql::*;
use redis::Commands;
use serde_json::{json, Value};

use crate::routes::functions::main_data::{
    // get_meteo_data,
    get_meteo_sky_data,
    get_near_obs_data,
    get_near_tide_data,
    get_near_wave_data,
    processing_data,
    get_warn_list
};


#[get("/data")]
pub async fn get_location_data(query: web::Query<RealLocation>) -> impl Responder {
    let mut db = DataBase::init();
    let mut conn = connect_redis();

    let lat: f64 = query.latitude.parse().expect("Error!");
    let lon: f64 = query.longitude.parse().expect("Error!");

    let obs_val: serde_json::Value = get_near_obs_data(&mut db, &mut conn, &lat, &lon);
    let wave_val: serde_json::Value = get_near_wave_data(&mut db, &mut conn, &lat, &lon);
    let tide_val: serde_json::Value = get_near_tide_data(&mut db, &mut conn, &lat, &lon);
    let meteo_val: serde_json::Value = get_meteo_sky_data(&mut db, &lat, &lon).await;

    web::Json(json!({
        "obs_data" : obs_val,
        "tidal" : tide_val,
        "wave_hight" : wave_val,
        "meteo_val" : meteo_val
    }))
}

#[get("/data/sky")]
pub async fn get_sky_data(query: web::Query<RealLocation>) -> impl Responder {
    let mut db = DataBase::init();

    let lat: f64 = query.latitude.parse().expect("Error!");
    let lon: f64 = query.longitude.parse().expect("Error!");

    let meteo_val: serde_json::Value = get_meteo_sky_data(&mut db, &lat, &lon).await;

    web::Json(json!({ "meteo_val": meteo_val }))
}

#[get("/region")]
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

#[get("/group")]
pub async fn group() -> impl Responder {
    let mut db = DataBase::init();

    let query = r"SELECT a.group_id, group_name, group_latitude, group_longitude, group_water_temp, group_salinity, group_height, group_weight, plain_buoy, COUNT(*) AS smart_buoy from buoy_group a, buoy_model b WHERE a.group_id = b.group_id AND a.group_id > 0 GROUP BY group_name";

    let row: Vec<MainGroupList> = db
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
                plain_buoy,
                smart_buoy,
            )| MainGroupList {
                group_id,
                group_name,
                group_latitude,
                group_longitude,
                group_water_temp,
                group_salinity,
                group_height,
                group_weight,
                plain_buoy,
                smart_buoy,
            },
        )
        .expect("select Error");

    let json = processing_data(&row, &mut db);

    web::Json(json)
}

#[get("/group/total")]
pub async fn group_total() -> impl Responder {
    let mut db = DataBase::init();

    let query = r"SELECT AVG(group_water_temp) AS water_temp, AVG(group_salinity) AS salinity, AVG(group_height) AS height, AVG(group_weight) AS weight FROM buoy_group";

    let row: Vec<Total> = db
        .conn
        .query_map(query, |(water_temp, salinity, height, weight)| Total {
            water_temp,
            salinity,
            height,
            weight,
        })
        .expect("select Error");

    web::Json(row)
}

#[get("/warn")]
pub async fn get_main_warn() -> impl Responder {
    let warn_list = get_warn_list();

    web::Json(warn_list)
}
