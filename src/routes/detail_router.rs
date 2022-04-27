use crate::db::maria_lib::DataBase;
use crate::db::redis_lib::connect_redis;
use crate::db::model::{Buoy, GroupList};

use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

use crate::routes::functions::detail_data::get_buoy_history;
use crate::routes::functions::detail_data::get_group_detail_data;
use crate::routes::functions::detail_data::get_buoy_list;
use crate::routes::functions::detail_data::get_group_history;

#[derive(Serialize)]
struct Obj {
    name: String,
}

#[get("/detail/group/list")]
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

#[get("/detail/group")]
pub async fn group_detail(query: web::Query<Name>) -> impl Responder {
    let val = get_group_detail_data(&query.group);

    web::Json(val)
}

#[get("/detail/group/history")]
pub async fn group_history(query: web::Query<Name>) -> impl Responder {
    let mut conn = connect_redis();
    let val = get_group_history(&query.group, &mut conn);

    web::Json(val)
}



#[derive(Deserialize, Serialize)]
pub struct BuoyListQuery {
    group: String,
}

#[get("/detail/buoy/list")]
pub async fn buoy_group_list(query: web::Query<BuoyListQuery>) -> impl Responder {

    let mut db = DataBase::init();

    let val = get_buoy_list(&query.group, &mut db);

    web::Json(val)
}


#[derive(Deserialize, Serialize)]
pub struct BuoyQuery {
    model: String,
}

#[get("/detail/buoy/history")]
pub async fn buoy_detail(query: web::Query<BuoyQuery>) -> impl Responder {
    let val = get_buoy_history(&query.model);

    web::Json(val)
}


