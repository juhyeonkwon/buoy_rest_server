use crate::db::maria_lib::DataBase;
use crate::db::model::{Buoy, GroupList};

use mysql::prelude::*;
use mysql::*;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Obj {
    name: String,
}

#[get("/detail/group/list")]
pub async fn group_list() -> impl Responder {
    let mut db = DataBase::init();

    let query = r"SELECT group_id, group_name FROM buoy_group";

    let row: Vec<GroupList> = db
        .conn
        .query_map(
            query,
            |(
                group_id,
                group_name,

            )| GroupList {
                group_id,
                group_name,

            },
        )
        .expect("select Error");

    web::Json(row)
}
