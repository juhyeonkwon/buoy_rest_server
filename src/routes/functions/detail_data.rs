
use redis::Commands;

use mysql::prelude::*;
use mysql::*;

use serde_json::{json, Value};

//1. 그룹안의 라인들의 평균값과 값 이력을 제공
//2. 각 그룹의 라인별 부이값들을 제공하면 될듯하다.

use crate::db::model::detail_model::{
    BuoyList, BuoySpecify, BuoyWarn, CheckGroup, GroupLineAvg, /*GroupModify, List,*/
};

use crate::db::model::main_model::MainGroupList;

pub fn get_group_detail_data(group_id: i32, user_idx: i32, maria_conn: &mut PooledConn, redis_conn: &mut redis::Connection) -> Vec<Value> {

    let mut json_vec: Vec<Value> = Vec::new();
    let temp: Vec<GroupLineAvg> = get_group_line_data(maria_conn, group_id, user_idx);

    for line in temp.iter() {
        let mut json = json!({});

        json["_line_info"] = json!(line);

        let history: Value = get_line_history(group_id, line.line, redis_conn);

        let buoys: Value = get_line_buoy_list(group_id, user_idx, line.line, maria_conn);

        json["_history"] = history;
        json["_buoy_list"] = buoys;

        json_vec.push(json);
    }

    json_vec
}

//라인별 평균값 제공
pub fn get_group_line_data(maria_conn: &mut PooledConn, group_id: i32, user_idx: i32) -> Vec<GroupLineAvg> {
    let stmt = 
    maria_conn
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

    let data: Vec<GroupLineAvg> = 
    maria_conn
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

pub fn get_group_history(group_id: i32, conn: &mut redis::Connection) -> Value {
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

pub fn get_line_buoy_list(group_id: i32, user_idx: i32, line: i16, maria_conn: &mut PooledConn) -> Value {
    let stmt = 
     maria_conn
        .prep(
            "SELECT model_idx, model, latitude, longitude, water_temp, salinity, height, weight, warn
             FROM
                 buoy_model a
             WHERE
                 a.group_id = :group_id AND line = :line AND user_idx = :idx
             ORDER BY model_idx asc",
        )
        .expect("Error");

    let data: Vec<BuoyList> = 
        maria_conn
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
pub fn get_buoy_history(model: &String, redis_conn: &mut redis::Connection) -> Value {

    let list: Vec<String> = redis::cmd("LRANGE")
        .arg(model)
        .arg("0")
        .arg("6")
        .query(redis_conn)
        .expect("Error!");

    let mut vec: Vec<Value> = Vec::new();

    for data in list {
        vec.push(serde_json::from_str(&data).expect("error!"));
    }

    serde_json::to_value(&vec).expect("Error!")
}

pub fn get_buoy(model: &String, maria_conn: &mut PooledConn) -> Value {
    let stmt = 
    maria_conn
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

    let data: Vec<BuoySpecify> = 
        maria_conn
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

    let stmt2 = 
    maria_conn
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

    let data2: Vec<BuoyWarn> = 
     maria_conn
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
pub fn get_buoy_list(group_id: i32, maria_conn: &mut PooledConn) -> Value {
    let stmt = 
        maria_conn
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

    let data: Vec<BuoySpecify> = 
        maria_conn
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

    let stmt2 = 
        maria_conn
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

    let data2: Vec<BuoyWarn> = 
        maria_conn
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
pub fn check_owned(maria_conn: &mut PooledConn, group_id: i32, user_idx: i32) -> usize {
    let stmt = maria_conn.prep("SELECT user_idx, group_id FROM buoy_group WHERE group_id = :group_id AND user_idx =  :idx").expect("Error!");

    let value: Vec<CheckGroup> = 
        maria_conn
        .exec_map(
            stmt,
            params! {
            "group_id" => group_id,
            "idx"      => user_idx
            },
            |(user_idx, group_id)| CheckGroup { user_idx, group_id },
        )
        .expect("DBError!");

    value.len()
}

pub fn check_owned_buoy(maria_conn: &mut PooledConn, model: &String, user_idx: i32) -> usize {
    let stmt = 
        maria_conn
        .prep("SELECT user_idx, group_id FROM buoy_model WHERE model = :model AND user_idx =  :idx")
        .expect("Error!");

    let value: Vec<CheckGroup> = 
        maria_conn
        .exec_map(
            stmt,
            params! {
            "model" => model,
            "idx"      => user_idx
            },
            |(user_idx, group_id)| CheckGroup { user_idx, group_id },
        )
        .expect("DBError!");

    value.len()
}

pub fn get_group_data(maria_conn: &mut PooledConn, group_id: i32, user_idx: i32) -> Value {
    let stmt = 
        maria_conn
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
                    WHERE a.group_id = b.group_id AND a.group_id = :group_id AND b.user_idx = :user_idx
                    GROUP BY a.group_id",
        )
        .expect("Error!");

    let row: Vec<MainGroupList> = 
        maria_conn
        .exec_map(
            stmt,
            params! {
                "group_id" => group_id,
                "user_idx" => user_idx
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

    if row.len() == 0 {
        json!({})
    } else {
        json!({"group_data" : processing_data(&row[0], maria_conn)})
    }
}

use crate::db::meteo::meteo_::Meteorological;

//디테일 그룹 데이터 프로세싱
pub fn processing_data(val: &MainGroupList, maria_conn: &mut PooledConn) -> Value {
    let mut temp: Value = serde_json::to_value(&val).expect("json parse error at group_list");

    let mut location = Meteorological::dfs_xy_conv(&val.group_latitude, &val.group_longitude);

    if location.x < 27.0 {
        temp["region"] = json!("미상");
    }

    let region = Meteorological::set_region_common(&mut location, maria_conn);

    temp["region"] = json!(region);

    temp
}
