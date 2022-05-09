use crate::db::maria_lib::DataBase;
// use crate::db::redis_lib::connect_redis;

// use crate::db::model::{Buoy, Group, MainGroupList};

use crate::db::model::detail_model::{BuoyQuery, UnassignedBuoy};

use actix_web::{delete, get, put, web, web::ReqData, Responder, Result};

use actix_web::error::ErrorUnauthorized;

use mysql::prelude::*;
use mysql::*;
// use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::custom_middleware::jwt::Claims;

use crate::routes::functions::auth::get_hash;

#[derive(Serialize, Deserialize)]
pub struct UserList {
    pub idx: i32,
    pub email: String,
    pub name: String,
    pub admin: i8,
}

#[get("/user/list")]
pub async fn user_list(token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let mut db = DataBase::init();

    let value: Vec<UserList> = db
        .conn
        .query_map(
            "SELECT idx, email, name, admin FROM users",
            |(idx, email, name, admin)| UserList {
                idx,
                email,
                name,
                admin,
            },
        )
        .expect("Error!");

    Ok(web::Json(value))
}

#[derive(Serialize, Deserialize)]
pub struct ModifyUser {
    idx: i32,
    name: String,
    password: String,
    admin: i32,
}

#[put("/user/modify")]
pub async fn user_modify(
    token: ReqData<Claims>,
    data: web::Json<ModifyUser>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let hash_pw = get_hash(&data.password);

    let mut db = DataBase::init();

    let stmt = db.conn.prep("UPDATE users set name = :name, password = :password, admin = :admin where idx = :user_idx").expect("PREP ERROR!");

    match db.conn.exec_drop(
        stmt,
        params! {
            "name" => &data.name,
            "password" => hash_pw,
            "idx" => data.idx,
            "admin" => data.admin
        },
    ) {
        Ok(_) => {
            let json = json!({ "code": 1 });
            Ok(web::Json(json))
        }
        Err(_) => {
            let json = json!({ "code": 0 });
            Ok(web::Json(json))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteUser {
    pub idx: i32,
}
#[delete("/user/delete")]
pub async fn user_delete(
    token: ReqData<Claims>,
    data: web::Json<DeleteUser>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let mut db = DataBase::init();

    let stmt = db
        .conn
        .prep("DELETE users where idx = :user_idx")
        .expect("PREP ERROR!");

    match db.conn.exec_drop(
        stmt,
        params! {
            "idx" => data.idx,
        },
    ) {
        Ok(_) => {
            let json = json!({ "code": 1 });
            Ok(web::Json(json))
        }
        Err(_) => {
            let json = json!({ "code": 0 });
            Ok(web::Json(json))
        }
    }
}

#[get("/buoy/unassigned")]
pub async fn buoy_unassigned(token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let mut db = DataBase::init();

    let query = "SELECT model_idx,
                                    model,
                                    latitude,
                                    longitude
                                    FROM 
                                        buoy_model 
                                    WHERE user_idx IS NULL";

    let value: Vec<UnassignedBuoy> = db
        .conn
        .query_map(query, |(model_idx, model, latitude, longitude)| {
            UnassignedBuoy {
                model_idx,
                model,
                latitude,
                longitude,
            }
        })
        .expect("db Error!");

    Ok(web::Json(value))
}

#[derive(Serialize, Deserialize)]
pub struct ManageBuoyAllocate {
    pub model: String,
    pub user_idx: i32,
}

#[put("/buoy/allocate")]
pub async fn buoy_allocate(
    token: ReqData<Claims>,
    buoy: web::Json<ManageBuoyAllocate>,
) -> impl Responder {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let stmt = db
        .conn
        .prep(
            "UPDATE buoy_model 
             SET 
                user_idx = :user_idx
             WHERE 
                model = :model",
        )
        .expect("Error!");

    match db.conn.exec_drop(
        stmt,
        params! {
            "user_idx" => buoy.user_idx,
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

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let stmt = db
        .conn
        .prep("UPDATE buoy_model set user_idx = null, group_id = 0, line = 0 where model = :model")
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
