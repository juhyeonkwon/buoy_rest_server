use crate::db::maria_lib::DataBase;
use crate::db::model::common_model::GroupList;
use crate::db::model::detail_model::{Obj, GroupId, BuoyQuery, BuoyAllocate};
use crate::db::redis_lib::connect_redis;

use actix_web::{get, put, web, /*HttpResponse, post,*/ Responder,  web::ReqData, Result};
use actix_web::error::ErrorUnauthorized;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::routes::functions::detail_data::{
    get_buoy, get_buoy_history, get_buoy_list, get_group_detail_data, get_group_history, check_owned, check_owned_buoy
};

use crate::custom_middleware::jwt::Claims;

#[get("/group/list")]
pub async fn group_list(token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    let stmt = db.conn.prep("SELECT group_id, group_name FROM buoy_group where group_id > 0 AND user_idx = :idx").expect("PREP ERROR");

    let row: Vec<GroupList> = db
        .conn
        .exec_map(stmt, params!{"idx" => user.idx}, |(group_id, group_name)| GroupList {
            group_id,
            group_name,
        })
        .expect("select Error");

    web::Json(row)
}


#[get("/group")]
pub async fn group_detail(token: ReqData<Claims>, query: web::Query<GroupId>) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();


    if check_owned(&mut db, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"))
    }
    
    let val = get_group_detail_data(query.group_id, user.idx, &mut db);

    Ok(web::Json(val))
}

#[get("/group/history")]
pub async fn group_history(token: ReqData<Claims>, query: web::Query<GroupId>) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();


    if check_owned(&mut db, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"))
    }

    let mut conn = connect_redis();
    let val = get_group_history(query.group_id, &mut conn);

    Ok(web::Json(val))
}


#[get("/buoy/list")]
pub async fn buoy_group_list(token: ReqData<Claims>, query: web::Query<GroupId>) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    
    if check_owned(&mut db, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"))
    }

    let val = get_buoy_list(query.group_id, &mut db);

    Ok(web::Json(val))
}


#[get("/buoy")]
pub async fn buoy_spec(token: ReqData<Claims>, query: web::Query<BuoyQuery>) -> Result<impl Responder> {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut db, &query.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"))
    }

    let val = get_buoy(&query.model, &mut db);

    Ok(web::Json(val))
}

#[get("/buoy/history")]
pub async fn buoy_detail(token: ReqData<Claims>, query: web::Query<BuoyQuery>) -> Result<impl Responder> {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut db, &query.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"))
    }

    let val = get_buoy_history(&query.model);

    Ok(web::Json(val))
}


#[put("/buoy/allocate")]
pub async fn buoy_allocate(token: ReqData<Claims>, buoy: web::Json<BuoyAllocate>) -> impl Responder {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut db, &buoy.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"))
    }

    if check_owned(&mut db, buoy.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"))
    }

    let stmt = db
        .conn
        .prep(
            "UPDATE buoy_model 
             SET 
                group_id = :group_id, 
                line = :line 
             WHERE 
                model = :model",
        )
        .expect("Error!");

    match db.conn.exec_drop(
        stmt,
        params! {
            "group_id" => &buoy.group_id,
            "line" => buoy.line,
            "model" => &buoy.model,
        },
    ) {
        Ok(_) => {
            let json = json!({"code" : 1});
            Ok(web::Json(json))
        }
        Err(_) => {
            let json = json!({"code" : 0});
            Ok(web::Json(json))
        }
    }
}

#[put("/buoy/deallocate")]
pub async fn buoy_deallocate(token: ReqData<Claims>, buoy: web::Json<BuoyQuery>) -> impl Responder {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut db, &buoy.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"))
    }

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
            Ok(web::Json(json))
        }
        Err(_) => {
            let json = json!({"code" : 0});
            Ok(web::Json(json))
        }
    }
}
