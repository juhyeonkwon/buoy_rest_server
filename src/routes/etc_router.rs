// use crate::db::maria_lib::DataBase;
// use crate::db::redis_lib::connect_redis;

// use crate::db::model::{Buoy, Group, MainGroupList};

use actix_web::{get, web, web::ReqData, Responder, Result};
// use mysql::prelude::*;
// use mysql::*;
// use redis::Commands;
// use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};

use crate::custom_middleware::jwt::Claims;
use crate::routes::functions::etc::get_warn_alarm_list;

#[get("/alarm/warn")]
pub async fn get_main_warn(redis : web::Data<redis::Client>) -> impl Responder {
    let mut redis_conn = redis.get_connection().unwrap();
    let warn_list = get_warn_alarm_list(&mut redis_conn);

    web::Json(warn_list)
}

#[get("/test")]
pub async fn get_test(token_option: ReqData<Claims>) -> Result<impl Responder> {
    let user: Claims = token_option.into_inner();

    // web::Json("\"msg\" : ok")
    Ok(web::Json(user))
}

