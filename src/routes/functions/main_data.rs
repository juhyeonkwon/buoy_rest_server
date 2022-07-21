use redis::Commands;

use crate::db::meteo::meteo_::Meteorological;
use crate::db::meteo::meteo_sky::MeteorologicalSky;
use crate::db::model::common_model::{TideBuoy, TideRader};

use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

/*

SELECT
(6371 * acos(cos(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0) * cos(:lat * 3.141592653589793 / 180.0)
* cos((:lon * 3.141592653589793 / 180.0) - (CAST(lon AS FLOAT) * 3.141592653589793 / 180.0)) + sin(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0)
* sin(:lat * 3.141592653589793 / 180.0))) as distance
FROM observation_list

 */

#[derive(Debug, Deserialize, Serialize)]
pub struct Distance {
    pub distance: f64,
    pub number: String,
    pub name: String,
}

pub fn get_near_obs_data(
    maria_conn: &mut PooledConn,
    redis_conn: &mut redis::Connection,
    lat: &f64,
    lon: &f64,
) -> Value {
    let stmt = maria_conn.prep("SELECT
  (6371 * acos(cos(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0) * cos(:lat * 3.141592653589793 / 180.0)
  * cos((:lon * 3.141592653589793 / 180.0) - (CAST(lon AS FLOAT) * 3.141592653589793 / 180.0)) + sin(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0)
  * sin(:lat * 3.141592653589793 / 180.0))) as distance, number, name
  FROM observation_list where tide_level = 1 AND w_temperature = 1 AND salinity = 1 AND air_temperature = 1 AND wind_velocity = 1 order BY distance asc").expect("Db prep Error!");

    let data: Vec<Distance> = 
    maria_conn
        .exec_map(
            stmt,
            params! {
              "lat" => lat,
              "lon" => lon,
            },
            |(distance, number, name)| Distance {
                distance,
                number,
                name,
            },
        )
        .expect("Error!");

    let _key = String::from("obs_") + &data[0].number;
    let mut a: String = String::from("");
    let _: () = match redis_conn.get(_key) {
        Ok(v) => a = v,
        Err(e) => println!("{}", e),
    };

    serde_json::from_str(&a).expect("parse Error!")
}

pub fn get_near_wave_data(
    maria_conn: &mut PooledConn,
    redis_conn: &mut redis::Connection,
    lat: &f64,
    lon: &f64,
) -> Value {
    let stmt = maria_conn.prep("SELECT
  (6371 * acos(cos(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0) * cos(:lat * 3.141592653589793 / 180.0)
  * cos((:lon * 3.141592653589793 / 180.0) - (CAST(lon AS FLOAT) * 3.141592653589793 / 180.0)) + sin(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0)
  * sin(:lat * 3.141592653589793 / 180.0))) as distance, number, name
  FROM observation_list where digging = 1 order BY distance asc").expect("Db prep Error!");

    let wave: Vec<Distance> = 
        maria_conn
        .exec_map(
            stmt,
            params! {
              "lat" => lat,
              "lon" => lon,
            },
            |(distance, number, name)| Distance {
                distance,
                number,
                name,
            },
        )
        .expect("redis Error!");


    let mut a: String = String::from("");

    for data in wave.iter() {
        let _key = String::from("wave_hight_") + &data.number;
        let _: () = match redis_conn.get(_key) {
            Ok(v) => {a = v; break},
            Err(e) => println!("redis Error!2 {}", e),
        };
    }

    serde_json::from_str(&a).expect("redis parse Error!")
}

pub fn get_near_tide_data(
    maria_conn: &mut PooledConn,
    redis_conn: &mut redis::Connection,
    lat: &f64,
    lon: &f64,
) -> Value {
    let stmt = maria_conn.prep("SELECT
  (6371 * acos(cos(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0) * cos(:lat * 3.141592653589793 / 180.0)
  * cos((:lon * 3.141592653589793 / 180.0) - (CAST(lon AS FLOAT) * 3.141592653589793 / 180.0)) + sin(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0)
  * sin(:lat * 3.141592653589793 / 180.0))) as distance, number, name
  FROM observation_list where tide_velocity > 0 order BY distance asc").expect("db prep Error!");

    let data: Vec<Distance> = 
        maria_conn
        .exec_map(
            stmt,
            params! {
              "lat" => lat,
              "lon" => lon,
            },
            |(distance, number, name)| Distance {
                distance,
                number,
                name,
            },
        )
        .expect("Error!");

    let mut a: String = String::from("");

    //HF인지 아닌지 체크
    let mut tide_type: String = String::from("");

    for val in data {
        let _key = String::from("tidal_") + &val.number;
        tide_type = String::from(&val.number[0..2]);
        let _: () = match redis_conn.get(_key) {
            Ok(v) => a = v,
            Err(_) => {
                println!("not found in redis");
                continue;
            }
        };
        break;
    }

    if tide_type == *"HF" {
        let list: Vec<TideRader> = serde_json::from_str(&a).expect("Error!");

        let value: &TideRader = get_neareast_hf(&list);

        return json!({
            "current_direct" : value.current_direct.parse::<f64>().expect("err"),
            "current_speed" : value.current_speed.parse::<f64>().expect("err"),
        });
    } else {
        let value: TideBuoy = serde_json::from_str(&a).expect("Error!");
        return json!({
            "current_direct" : value.current_direct.parse::<f64>().expect("err"),
            "current_speed" : value.current_speed.parse::<f64>().expect("err"),
        });
    }
}

fn get_neareast_hf(list: &[TideRader]) -> &TideRader {
    let mut min: f64 = 300.0;
    let mut current = 0;

    for (i, val) in list.iter().enumerate() {
        let dis: f64 = get_distance(
            (35.1513466, 128.1001125),
            (val.lat.parse().expect("Err"), val.lon.parse().expect("Err")),
        );

        if dis < min {
            min = dis;
            current = i;
        }
    }

    &list[current]
}

// pub async fn get_meteo_data(db: &mut DataBase, lat: &f64, lon: &f64) -> Value {
//     let _key: String = match env::var("GEO_KEY") {
//         Ok(v) => v,
//         Err(_) => panic!("Env GEO_KEY Not Found!"),
//     };

//     let obj = Meteorological::init(db, &lat, &lon).await;

//     println!("getmoeto");

//     let value: Value = serde_json::to_value(obj).expect("Error!");

//     json!({
//         "data" : value["data"],
//         "region" : value["region"]
//     })

//     //url 정의
// }

pub async fn get_meteo_sky_data(maria_conn: &mut PooledConn, redis_conn : &mut redis::Connection, lat: &f64, lon: &f64) -> Value {
    let _key: String = match env::var("GEO_KEY") {
        Ok(v) => v,
        Err(_) => panic!("Env GEO_KEY Not Found!"),
    };

    let obj = MeteorologicalSky::init(maria_conn, redis_conn, &lat, &lon).await;

    let data : Value;


    if obj.data.len() == 0 {
        data = json!({
            "humidity": "0",
            "rain_amt_hour": "강수없음",
            "rain_code": "0",
            "sky": "0",
            "sn_wind": "0",
            "temperature": "0",
            "thunder": "0",
            "we_wind": "0",
            "wind_direction": "0",
            "wind_velocity": "0"
        });
    } else {
        data = obj.get_json_value();
    }


    let value: Value = serde_json::to_value(obj).expect("Error!");

    json!({
        "data" : data,
        "region" : value["region"]
    })

    //url 정의
}

//지구상 두 좌표사이의 거리를 제공
fn get_distance(center: (f64, f64), target: (f64, f64)) -> f64 {
    let earth_radius_kilometer = 6371.0_f64;
    let (center_latitude_degrees, center_longitude_degrees) = center;
    let (target_latitude_degrees, target_longitude_degrees) = target;

    let center_latitude = center_latitude_degrees.to_radians();
    let target_latitude = target_latitude_degrees.to_radians();

    let delta_latitude = (center_latitude_degrees - target_latitude_degrees).to_radians();
    let delta_longitude = (center_longitude_degrees - target_longitude_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + center_latitude.cos() * target_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    earth_radius_kilometer * central_angle
}

use crate::db::model::main_model::MainGroupList;
//메인 그룹 데이터 프로세싱
pub fn processing_data(vec: &Vec<MainGroupList>, conn : &mut PooledConn) -> Vec<Value> {
    let mut json: Vec<Value> = Vec::new();

    for val in vec.iter() {
        let mut temp: Value = serde_json::to_value(&val).expect("json parse error at group_list");
        
        if temp["group_latitude"] == 0.0 || temp["group_longitude"] == 0.0 {
            temp["region"] = json!("미상");
            json.push(temp);
            continue;
        } 

        let mut location = Meteorological::dfs_xy_conv(&val.group_latitude, &val.group_longitude);

        if location.x < 27.0 {
            temp["region"] = json!("미상");
            continue;
        }

        let region = Meteorological::set_region_common(&mut location, conn);

        temp["region"] = json!(region);

        json.push(temp);
    }

    json
}

use crate::db::model::common_model::Warn;

pub fn get_warn_list(user_idx: i32, conn : &mut redis::Connection) -> Vec<Warn> {

    let warn_text: String = match redis::cmd("GET").arg("warn_list").query(conn) {
        Ok(v) => v,
        Err(_) => String::from("{}"),
    };

    let mut return_vec: Vec<Warn> = Vec::new();

    let vec: Vec<Warn> = match serde_json::from_str(&warn_text) {
        Ok(v) => v,
        Err(_) => Vec::new(),
    };

    for val in vec.iter() {
        if val.user_idx == user_idx {
            return_vec.push(val.clone());
        }
    }

    return_vec
}
