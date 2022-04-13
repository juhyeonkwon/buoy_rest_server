use crate::db::maria_lib::DataBase;
use crate::db::redis_lib::connect_redis;

use crate::db::model::{Buoy, Group};

use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize)]
struct Obj {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub location: String,
}

#[get("/main")]
pub async fn get_main_data(query: web::Query<Location>) -> impl Responder {
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
