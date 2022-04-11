use crate::db::maria_lib::DataBase;
use crate::db::model::{Buoy, Group};

use mysql::prelude::*;
use mysql::*;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Obj {
    name: String,
}

#[get("/main")]
pub async fn get_main_data() -> impl Responder {
    let obj = Obj {
        name: String::from("abc"),
    };

    web::Json(obj)
}

#[get("/main/total")]
pub async fn get_main_total() -> impl Responder {
    let obj = Obj {
        name: String::from("abc"),
    };

    web::Json(obj)
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
