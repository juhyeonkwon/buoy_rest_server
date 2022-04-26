// use crate::db::maria_lib::DataBase;
// use crate::db::redis_lib::connect_redis;

// use crate::db::model::{Buoy, Group, MainGroupList};

use actix_web::{get, web, Responder};
// use mysql::prelude::*;
// use mysql::*;
// use redis::Commands;
// use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};

use crate::routes::functions::etc::get_warn_alarm_list;

#[get("/alarm/warn")]
pub async fn get_main_warn() -> impl Responder {

    let warn_list = get_warn_alarm_list();

    web::Json(warn_list)
}
