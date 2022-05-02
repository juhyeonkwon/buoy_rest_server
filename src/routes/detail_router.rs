use crate::db::maria_lib::DataBase;
use crate::db::model::common_model::{GroupList};
use crate::db::redis_lib::connect_redis;

use actix_web::{get, put, web, /*HttpResponse, post,*/ Responder};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::routes::functions::detail_data::get_buoy;
use crate::routes::functions::detail_data::get_buoy_history;
use crate::routes::functions::detail_data::get_buoy_list;
use crate::routes::functions::detail_data::get_group_detail_data;
use crate::routes::functions::detail_data::get_group_history;

#[derive(Serialize)]
struct Obj {
    name: String,
}

#[get("/group/list")]
pub async fn group_list() -> impl Responder {
    let mut db = DataBase::init();

    let query = r"SELECT group_id, group_name FROM buoy_group where group_id > 0";

    let row: Vec<GroupList> = db
        .conn
        .query_map(query, |(group_id, group_name)| GroupList {
            group_id,
            group_name,
        })
        .expect("select Error");

    web::Json(row)
}

#[derive(Deserialize, Serialize)]
pub struct Name {
    pub group: String,
}

#[get("/group")]
pub async fn group_detail(query: web::Query<Name>) -> impl Responder {
    let val = get_group_detail_data(&query.group);

    web::Json(val)
}

#[get("/group/history")]
pub async fn group_history(query: web::Query<Name>) -> impl Responder {
    let mut conn = connect_redis();
    let val = get_group_history(&query.group, &mut conn);

    web::Json(val)
}

#[derive(Deserialize, Serialize)]
pub struct BuoyListQuery {
    group: String,
}

#[get("/buoy/list")]
pub async fn buoy_group_list(query: web::Query<BuoyListQuery>) -> impl Responder {
    let mut db = DataBase::init();

    let val = get_buoy_list(&query.group, &mut db);

    web::Json(val)
}

#[derive(Deserialize, Serialize)]
pub struct BuoyQuery {
    model: String,
}

#[get("/buoy")]
pub async fn buoy_spec(query: web::Query<BuoyQuery>) -> impl Responder {
    let mut db = DataBase::init();

    let val = get_buoy(&query.model, &mut db);

    web::Json(val)
}

#[get("/buoy/history")]
pub async fn buoy_detail(query: web::Query<BuoyQuery>) -> impl Responder {
    let val = get_buoy_history(&query.model);

    web::Json(val)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BuoyAllocate {
    model: String,
    group_name: String,
    line: i8,
}

#[put("/buoy/allocate")]
pub async fn buoy_allocate(buoy: web::Form<BuoyAllocate>) -> impl Responder {
    let mut db = DataBase::init();

    println!("{:#?}buoy", buoy);

    let stmt = db
        .conn
        .prep(
            "UPDATE buoy_model 
             SET 
                group_id = 
                     (SELECT group_id FROM buoy_group 
                      WHERE   
                        group_name = :group_name), 
                line = :line 
             WHERE 
                model = :model",
        )
        .expect("Error!");

    match db.conn.exec_drop(
        stmt,
        params! {
            "group_name" => &buoy.group_name,
            "line" => buoy.line,
            "model" => &buoy.model,
        },
    ) {
        Ok(_) => {
            let json = json!({"code" : 1});
            web::Json(json)
        }
        Err(_) => {
            let json = json!({"code" : 0});
            web::Json(json)
        }
    }
}

#[put("/buoy/deallocate")]
pub async fn buoy_deallocate(buoy: web::Form<BuoyQuery>) -> impl Responder {
    let mut db = DataBase::init();

    let stmt = db
        .conn
        .prep("UPDATE buoy_model set group_id = 0, line = 0 where model = :model")
        .expect("Error!");

    match db.conn.exec_drop(
        stmt,
        params! {
            "model" => &buoy.model,
        },
    ) {
        Ok(_) => {
            let json = json!({"code" : 1});
            web::Json(json)
        }
        Err(_) => {
            let json = json!({"code" : 0});
            web::Json(json)
        }
    }
}
