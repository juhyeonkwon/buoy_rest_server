// use crate::db::maria_lib::DataBase;
// use crate::db::redis_lib::connect_redis;

// use crate::db::model::{Buoy, Group, MainGroupList};

use actix_web::{get, http::header::ContentType, web, web::ReqData, HttpResponse, Responder};
// use mysql::prelude::*;
// use mysql::*;
// use redis::Commands;
// use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};

use crate::custom_middleware::jwt::{get_user_claim, Claims};
use crate::routes::functions::etc::get_warn_alarm_list;

#[get("/alarm/warn")]
pub async fn get_main_warn() -> impl Responder {
    let warn_list = get_warn_alarm_list();

    web::Json(warn_list)
}

#[get("/test")]
pub async fn get_test(token_option: Option<ReqData<Option<Claims>>>) -> impl Responder {
    let user: Claims;

    match get_user_claim(token_option) {
        Some(v) => user = v,
        None => {
            return HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body("{ \"code\" : 0}")
        }
    };

    println!("{:#?}", user);

    let txt = serde_json::to_string(&user).expect("Error!");

    // web::Json("\"msg\" : ok")
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(txt)
}
