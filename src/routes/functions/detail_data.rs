use crate::db::maria_lib::DataBase;
use crate::db::redis_lib::connect_redis;
use redis::Commands;

use mysql::prelude::*;
use mysql::*;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

use chrono;
use chrono::prelude::*;
use chrono::Duration;

//1. 그룹안의 라인들의 평균값과 값 이력을 제공
//2. 각 그룹의 라인별 부이값들을 제공하면 될듯하다.

/*

SELECT b.group_name,
      line,
      AVG(latitude) as latitude,
      AVG(longitude) as longitude,
      AVG(water_temp) as water_temp,
      AVG(salinity) as salinity,
      AVG(height) as height,
      AVG(weight) as weight
FROM
      buoy_model a
INNER JOIN
      buoy_group b ON a.group_id = b.group_id
WHERE
       group_name = :NAME GROUP BY a.line

*/

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupLineAvg {
    pub group_name: String,
    pub line: i16,
    pub latitude: f64,
    pub longitude: f64,
    pub water_temp: f64,
    pub salinity: f64,
    pub height: f64,
    pub weight: f64,
}

pub fn get_group_detail_data(name: &String) -> Vec<Value> {
    let mut db = DataBase::init();
    let mut conn = connect_redis();

    let mut json_vec: Vec<Value> = Vec::new();
    let temp: Vec<GroupLineAvg> = get_group_line_data(&mut db, &name);

    for line in temp.iter() {
        let mut json = json!({});

        json["_line_info"] = json!(line);

        let history: Value = get_line_history(&name, line.line, &mut conn);

        let buoys: Value = get_line_buoy_list(&name, line.line, &mut db);

        json["_history"] = history;
        json["_buoy_list"] = buoys;

        json_vec.push(json);
    }

    json_vec
}

//라인별 평균값 제공
pub fn get_group_line_data(db: &mut DataBase, name: &String) -> Vec<GroupLineAvg> {
    let stmt = db
        .conn
        .prep(
            "SELECT b.group_name,
                line,
                AVG(latitude) as latitude,
                AVG(longitude) as longitude,
                AVG(water_temp) as water_temp,
                AVG(salinity) as salinity,
                AVG(height) as height,
                AVG(weight) as weight
            FROM
                buoy_model a
            INNER JOIN
                buoy_group b ON a.group_id = b.group_id
            WHERE
                group_name = :name GROUP BY a.line",
        )
        .expect("stmt Error!");

    let data: Vec<GroupLineAvg> = db
        .conn
        .exec_map(
            stmt,
            params! {
              "name" => name
            },
            |(group_name, line, latitude, longitude, water_temp, salinity, height, weight)| {
                GroupLineAvg {
                    group_name,
                    line,
                    latitude,
                    longitude,
                    water_temp,
                    salinity,
                    height,
                    weight,
                }
            },
        )
        .expect("error");

    data
}

pub struct List {
    pub group_id: i16,
    pub group_name: String,
}

pub fn get_group_history(name: &String, conn: &mut redis::Connection) -> Value {
    let key: String = String::from(name) + "_group";

    let list: Vec<String> = redis::cmd("LRANGE")
        .arg(&key)
        .arg("0")
        .arg("6")
        .query(conn)
        .expect("Error!");

    let mut vec: Vec<Value> = Vec::new();

    for data in list {
        vec.push(serde_json::from_str(&data).expect("error!"));
    }

    serde_json::to_value(&vec).expect("Error!")
}

pub fn get_line_history(name: &String, line: i16, conn: &mut redis::Connection) -> Value {
    let key: String = String::from(name) + "_group_line_" + &line.to_string();

    let list: Vec<String> = redis::cmd("LRANGE")
        .arg(&key)
        .arg("0")
        .arg("6")
        .query(conn)
        .expect("Error!");

    let mut vec: Vec<Value> = Vec::new();

    for data in list {
        vec.push(serde_json::from_str(&data).expect("error!"));
    }

    serde_json::to_value(&vec).expect("Error!")
}

#[derive(Serialize)]
pub struct BuoyList {
    pub model_idx: i16,
    pub model: String,
    pub latitude: f64,
    pub longitude: f64,
    pub water_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
    pub warn: i16,
}

pub fn get_line_buoy_list(name: &String, line: i16, db: &mut DataBase) -> Value {
    let stmt = db
        .conn
        .prep(
            "SELECT model_idx, model, latitude, longitude, water_temp, salinity, height, weight, warn
             FROM
                 buoy_model a
             INNER JOIN
                 buoy_group b ON a.group_id = b.group_id
             WHERE
                 group_name = :group_name AND line = :line
             ORDER BY model_idx asc",
        )
        .expect("Error");

    let data: Vec<BuoyList> = db
        .conn
        .exec_map(
            stmt,
            params! {
                "group_name" => name,
                "line" => line
            },
            |(
                model_idx,
                model,
                latitude,
                longitude,
                water_temp,
                salinity,
                height,
                weight,
                warn,
            )| BuoyList {
                model_idx,
                model,
                latitude,
                longitude,
                water_temp,
                salinity,
                height,
                weight,
                warn,
            },
        )
        .expect("DB Error!");

    serde_json::to_value(&data).expect("Error!")
}

#[derive(Debug, Serialize)]
struct BuoyCurrent {}

//부이의 7일간 히스토리 가져온다람쥐
pub fn get_buoy_history(model: &String) -> Value {
    let mut conn = connect_redis();

    let list: Vec<String> = redis::cmd("LRANGE")
        .arg(model)
        .arg("0")
        .arg("6")
        .query(&mut conn)
        .expect("Error!");

    let mut vec: Vec<Value> = Vec::new();

    for data in list {
        vec.push(serde_json::from_str(&data).expect("error!"));
    }

    serde_json::to_value(&vec).expect("Error!")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuoySpecify {
    pub model_idx: i16,
    pub model: String,
    pub line: i8,
    pub group_id: i8,
    pub group_name : String,
    pub latitude: f64,
    pub longitude: f64,
    pub water_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
    pub warn: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuoyWarn {
    pub temp_warn: i8,
    pub salinity_warn: i8,
    pub height_warn : i8,
    pub weight_warn: i8,
    pub location_warn: i8,
}

//Buoy의 그룹별 모든 리스트 줌
pub fn get_buoy_list(group : &String, db : &mut DataBase) -> Value {
    let stmt = db
        .conn
        .prep(
            "SELECT model_idx, model, line, a.group_id, b.group_name, latitude, longitude, water_temp, salinity, height, weight, warn
             FROM
                 buoy_model a
             INNER JOIN
                 buoy_group b ON a.group_id = b.group_id
             WHERE
                 group_name = :group_name
             ORDER BY model_idx asc",
        )
        .expect("Error");

    let data: Vec<BuoySpecify> = db
        .conn
        .exec_map(
            stmt,
            params! {
                "group_name" => group,
            },
            |(
                model_idx,
                model,
                line,
                group_id,
                group_name,
                latitude,
                longitude,
                water_temp,
                salinity,
                height,
                weight,
                warn,
            )| BuoySpecify {
                model_idx,
                model,
                line,
                group_id,
                group_name,
                latitude,
                longitude,
                water_temp,
                salinity,
                height,
                weight,
                warn,
            },
        )
        .expect("DB Error!");

    let stmt2 = db
        .conn
        .prep(
            "SELECT temp_warn, salinity_warn, height_warn, weight_warn, location_warn
            FROM
                buoy_model a
            LEFT OUTER JOIN
                buoy_group b ON a.group_id = b.group_id
            WHERE
                group_name = :group_name
            ORDER BY model_idx asc",
        )
        .expect("Error");

    let data2: Vec<BuoyWarn> = db
        .conn
        .exec_map(
            stmt2,
            params! {
                "group_name" => group,
            },
            |(
                temp_warn,
                salinity_warn,
                height_warn ,
                weight_warn,
                location_warn,
            )| BuoyWarn {
                temp_warn,
                salinity_warn,
                height_warn ,
                weight_warn,
                location_warn,
            },
        )
        .expect("DB Error!");

    let mut json : Vec<Value> = Vec::new();

    for (i, val) in data.iter().enumerate() {
        let mut temp : Value = serde_json::to_value(&val).expect("Error!");
        temp["warn_detail"] = json!(data2[i]);
        json.push(temp);
    }


    serde_json::to_value(&json).expect("Error!")

}