// use crate::db::maria_lib::DataBase;
// use crate::db::redis_lib::connect_redis;
// use actix_web::error::ErrorUnauthorized;

use crate::db::model::main_model::{Location, MainGroupList, RealLocation, Total};
use actix_web::{get, /*post,*/ web, web::ReqData, /*HttpResponse,*/ Responder, Result};
use mysql::prelude::*;
use mysql::*;
use redis::Commands;
use serde_json::{json, Value};

use crate::custom_middleware::jwt::Claims;
use crate::routes::functions::main_data::{
    // get_meteo_data,
    get_meteo_sky_data,
    get_near_obs_data,
    get_near_tide_data,
    get_near_wave_data,
    get_warn_list,
    processing_data,
};

#[get("/data")]
pub async fn get_location_data(pool : web::Data<mysql::Pool>, redis : web::Data<redis::Client>, query: web::Query<RealLocation>) -> Result<impl Responder> {
    let mut maria_conn = pool.get_conn().unwrap();
    let mut redis_conn = redis.get_connection().expect("faild to connect to Redis.");

    let lat: f64 = query.latitude.parse().expect("Error!");
    let lon: f64 = query.longitude.parse().expect("Error!");

    let obs_val: serde_json::Value = get_near_obs_data(&mut maria_conn, &mut redis_conn, &lat, &lon);
    let wave_val: serde_json::Value = get_near_wave_data(&mut maria_conn, &mut redis_conn, &lat, &lon);
    let tide_val: serde_json::Value = get_near_tide_data(&mut maria_conn, &mut redis_conn, &lat, &lon);
    let meteo_val: serde_json::Value = get_meteo_sky_data(&mut maria_conn, &mut redis_conn, &lat, &lon).await;

    Ok(web::Json(json!({
        "obs_data" : obs_val,
        "tidal" : tide_val,
        "wave_hight" : wave_val,
        "meteo_val" : meteo_val
    })))
}

#[get("/data/sky")]
pub async fn get_sky_data(pool : web::Data<mysql::Pool>, redis : web::Data<redis::Client>, query: web::Query<RealLocation>) -> impl Responder {

    let mut conn = pool.get_conn().unwrap();
    let mut redis_conn = redis.get_connection().unwrap();

    let lat: f64 = query.latitude.parse().expect("Error!");
    let lon: f64 = query.longitude.parse().expect("Error!");

    let meteo_val: serde_json::Value = get_meteo_sky_data(&mut conn, &mut redis_conn, &lat, &lon).await;

    web::Json(json!({ "meteo_val": meteo_val }))
}

#[get("/region")]
pub async fn get_main_data_region(redis : web::Data<redis::Client>, query: web::Query<Location>) -> impl Responder {
    let mut conn = redis.get_connection().expect("faild to connect to Redis.");
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
pub async fn group(pool : web::Data<mysql::Pool>, token_option: ReqData<Claims>) -> impl Responder {
    let user: Claims = token_option.into_inner();

    let mut conn = pool.get_conn().unwrap();

    let stmt = 
        conn
        .prep(
            "SELECT a.group_id, 
                    group_name, 
                    group_latitude, 
                    group_longitude, 
                    group_water_temp, 
                    group_salinity, 
                    group_height, 
                    group_weight, 
                    group_system,
                    plain_buoy, 
                    COUNT(b.model_idx) AS smart_buoy 
                    from buoy_group a, buoy_model b 
                    WHERE a.group_id = b.group_id AND a.group_id > 0 AND b.user_idx = :user_idx
                    GROUP BY a.group_id",
        )
        .expect("Error!");

    let row: Vec<MainGroupList> = 
        conn
        .exec_map(
            stmt,
            params! {
                "user_idx" => user.idx
            },
            |(
                group_id,
                group_name,
                group_latitude,
                group_longitude,
                group_water_temp,
                group_salinity,
                group_height,
                group_weight,
                group_system,
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
                group_system,
                plain_buoy,
                smart_buoy,
            },
        )
        .expect("select Error");

    let json = processing_data(&row, &mut conn);

    web::Json(json)
}

#[get("/group/total")]
pub async fn group_total(pool : web::Data<mysql::Pool>, token_option: ReqData<Claims>) -> impl Responder {
    let user: Claims = token_option.into_inner();

    // let mut db = DataBase::init();

    // let query = r"SELECT AVG(group_water_temp) AS water_temp, AVG(group_salinity) AS salinity, AVG(group_height) AS height, AVG(group_weight) AS weight FROM buoy_group";

    // let row: Vec<Total> = db
    //     .conn
    //     .query_map(query, |(water_temp, salinity, height, weight)| Total {
    //         water_temp,
    //         salinity,
    //         height,
    //         weight,
    //     })
    //     .expect("select Error");
    let mut conn = pool.get_conn().unwrap();

    let stmt = 
        conn
        .prep(
            "SELECT     COUNT(group_id) AS group_count,
                        CAST(IFNULL(AVG(group_water_temp), 0.0) AS FLOAT) AS water_temp, 
                        CAST(IFNULL(AVG(group_salinity), 0.0) AS FLOAT) AS salinity, 
                        CAST(IFNULL(AVG(group_height), 0.0) AS FLOAT) AS height, 
                        CAST(IFNULL(AVG(group_weight), 0.0) AS FLOAT) AS weight, 
                        CAST(IFNULL(SUM(plain_buoy), 0) AS INT) AS plain_buoy,
                        (SELECT COUNT(model_idx) FROM buoy_model WHERE user_idx = 1) AS smart_buoy 
                        FROM buoy_group WHERE user_idx = :idx AND group_id > 0;",
        )
        .expect("Error!");

    let row: Vec<Total> = match conn.exec_map(
        stmt,
        params! {"idx" => user.idx },
        |(group_count, water_temp, salinity, height, weight, plain_buoy, smart_buoy)| Total {
            group_count,
            water_temp,
            salinity,
            height,
            weight,
            plain_buoy,
            smart_buoy,
        },
    ) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            Vec::new()
        }
    };

    web::Json(row)
}

#[get("/warn")]
pub async fn get_main_warn(redis : web::Data<redis::Client>,token_option: ReqData<Claims>) -> impl Responder {
    let user: Claims = token_option.into_inner();

    let mut redis_conn = redis.get_connection().unwrap();
    let warn_list = get_warn_list(user.idx, &mut redis_conn);

    web::Json(warn_list)
}
