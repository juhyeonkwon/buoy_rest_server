use crate::db::maria_lib::DataBase;
use crate::db::redis_lib::connect_redis;
use redis::Commands;

use mysql::prelude::*;
use mysql::*;

use serde_json::{json, Value};

//1. 그룹안의 라인들의 평균값과 값 이력을 제공
//2. 각 그룹의 라인별 부이값들을 제공하면 될듯하다.

use crate::db::model::detail_model::{BuoyList, BuoySpecify, BuoyWarn, GroupLineAvg, List, CheckGroup};

pub fn get_group_detail_data(group_id: i32, user_idx : i32, db : &mut DataBase) -> Vec<Value> {
    let mut conn = connect_redis();

    let mut json_vec: Vec<Value> = Vec::new();
    let temp: Vec<GroupLineAvg> = get_group_line_data(db, group_id, user_idx);

    for line in temp.iter() {
        let mut json = json!({});

        json["_line_info"] = json!(line);

        let history: Value = get_line_history(group_id, line.line, &mut conn);

        let buoys: Value = get_line_buoy_list(group_id, user_idx, line.line, db);

        json["_history"] = history;
        json["_buoy_list"] = buoys;

        json_vec.push(json);
    }

    json_vec
}

//라인별 평균값 제공
pub fn get_group_line_data(db: &mut DataBase, group_id : i32, user_idx : i32) -> Vec<GroupLineAvg> {
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
                a.group_id = :group_id AND a.user_idx = :idx GROUP BY a.line",
        )
        .expect("stmt Error!");

    let data: Vec<GroupLineAvg> = db
        .conn
        .exec_map(
            stmt,
            params! {
              "group_id" => group_id,
              "idx" => user_idx
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

pub fn get_group_history(group_id : i32, conn: &mut redis::Connection) -> Value {
    let key: String = String::from(group_id.to_string()) + "_group";

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

pub fn get_line_history(group_id: i32, line: i16, conn: &mut redis::Connection) -> Value {
    let key: String = String::from(group_id.to_string()) + "_group_line_" + &line.to_string();

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

pub fn get_line_buoy_list(group_id : i32, user_idx : i32, line: i16, db: &mut DataBase) -> Value {
    let stmt = db
        .conn
        .prep(
            "SELECT model_idx, model, latitude, longitude, water_temp, salinity, height, weight, warn
             FROM
                 buoy_model a
             WHERE
                 a.group_id = :group_id AND line = :line AND user_idx = :idx
             ORDER BY model_idx asc",
        )
        .expect("Error");

    let data: Vec<BuoyList> = db
        .conn
        .exec_map(
            stmt,
            params! {
                "group_id" => group_id,
                "line" => line,
                "idx" => user_idx
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

pub fn get_buoy(model: &String, db: &mut DataBase) -> Value {
    let stmt = db
        .conn
        .prep(
            "SELECT model_idx, model, line, a.group_id, b.group_name, latitude, longitude, water_temp, salinity, height, weight, warn
             FROM
                 buoy_model a
             INNER JOIN
                 buoy_group b ON a.group_id = b.group_id
             WHERE
                a.model = :model
             ORDER BY model_idx asc",
        )
        .expect("Error");

    let data: Vec<BuoySpecify> = db
        .conn
        .exec_map(
            stmt,
            params! {
                "model" => model,
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
            model = :model",
        )
        .expect("Error");

    let data2: Vec<BuoyWarn> = db
        .conn
        .exec_map(
            stmt2,
            params! {
                "model" => model,
            },
            |(temp_warn, salinity_warn, height_warn, weight_warn, location_warn)| BuoyWarn {
                temp_warn,
                salinity_warn,
                height_warn,
                weight_warn,
                location_warn,
            },
        )
        .expect("DB Error!");

    let mut json: Vec<Value> = Vec::new();

    for (i, val) in data.iter().enumerate() {
        let mut temp: Value = serde_json::to_value(&val).expect("Error!");
        temp["warn_detail"] = json!(data2[i]);
        json.push(temp);
    }

    serde_json::to_value(&json).expect("Error!")
}

//Buoy의 그룹별 모든 리스트 줌
pub fn get_buoy_list(group_id: i32, db: &mut DataBase) -> Value {
    let stmt = db
        .conn
        .prep(
            "SELECT model_idx, model, line, a.group_id, b.group_name, latitude, longitude, water_temp, salinity, height, weight, warn
             FROM
                 buoy_model a
             INNER JOIN
                 buoy_group b ON a.group_id = b.group_id
             WHERE
                 a.group_id = :group_id
             ORDER BY model_idx asc",
        )
        .expect("Error");

    let data: Vec<BuoySpecify> = db
        .conn
        .exec_map(
            stmt,
            params! {
                "group_id" => group_id,
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
                a.group_id = :group_id
            ORDER BY model_idx asc",
        )
        .expect("Error");

    let data2: Vec<BuoyWarn> = db
        .conn
        .exec_map(
            stmt2,
            params! {
                "group_id" => group_id,
            },
            |(temp_warn, salinity_warn, height_warn, weight_warn, location_warn)| BuoyWarn {
                temp_warn,
                salinity_warn,
                height_warn,
                weight_warn,
                location_warn,
            },
        )
        .expect("DB Error!");

    let mut json: Vec<Value> = Vec::new();

    for (i, val) in data.iter().enumerate() {
        let mut temp: Value = serde_json::to_value(&val).expect("Error!");
        temp["warn_detail"] = json!(data2[i]);
        json.push(temp);
    }

    serde_json::to_value(&json).expect("Error!")
}



//user_id에 맞는 그룹인지 확인하는 함수람쥐
pub fn check_owned(db : &mut DataBase, group_id : i32, user_idx : i32) -> usize {

    let stmt = db.conn.prep("SELECT user_idx, group_id FROM buoy_group WHERE group_id = :group_id AND user_idx =  :idx").expect("Error!");

    let value : Vec<CheckGroup> = db.conn.exec_map(stmt, params!{
                                    "group_id" => group_id,
                                    "idx"      => user_idx  
                                    }, 
                                    |(user_idx, group_id)| CheckGroup {
                                        user_idx, 
                                        group_id 
                                    }).expect("DBError!");


    value.len()
    

}


pub fn check_owned_buoy(db : &mut DataBase, model : &String, user_idx : i32) -> usize {
    let stmt = db.conn.prep("SELECT user_idx, group_id FROM buoy_model WHERE model = :model AND user_idx =  :idx").expect("Error!");

    let value : Vec<CheckGroup> = db.conn.exec_map(stmt, params!{
                                    "model" => model,
                                    "idx"      => user_idx  
                                    }, 
                                    |(user_idx, group_id)| CheckGroup {
                                        user_idx, 
                                        group_id 
                                    }).expect("DBError!");


    value.len()
}