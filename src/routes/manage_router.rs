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
pub async fn user_list( pool: web::Data<Pool>, token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();


    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let mut maria_conn = pool.get_conn().unwrap();

    let value: Vec<UserList> = maria_conn
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
    pool: web::Data<Pool>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let hash_pw = get_hash(&data.password);

    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = maria_conn.prep("UPDATE users set name = :name, password = :password, admin = :admin where idx = :user_idx").expect("PREP ERROR!");

    match maria_conn.exec_drop(
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
    pool: web::Data<Pool>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = maria_conn
        .prep("DELETE users where idx = :user_idx")
        .expect("PREP ERROR!");

    match maria_conn.exec_drop(
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
pub async fn buoy_unassigned(pool: web::Data<Pool>, token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let mut maria_conn = pool.get_conn().unwrap();

    let query = "SELECT model_idx,
                                    model,
                                    latitude,
                                    longitude
                                    FROM 
                                        buoy_model 
                                    WHERE user_idx IS NULL";

    let value: Vec<UnassignedBuoy> = maria_conn
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
    pool: web::Data<Pool>,
) -> impl Responder {

    let user: Claims = token.into_inner();


    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = maria_conn
        .prep(
            "UPDATE buoy_model 
             SET 
                user_idx = :user_idx
             WHERE 
                model = :model",
        )
        .expect("Error!");

    match maria_conn.exec_drop(
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
pub async fn buoy_deallocate(pool: web::Data<Pool>, token: ReqData<Claims>, buoy: web::Json<BuoyQuery>) -> impl Responder {

    let user: Claims = token.into_inner();

    if user.admin == 0 {
        return Err(ErrorUnauthorized("Not Admin"));
    }

    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = maria_conn
        .prep("UPDATE buoy_model set user_idx = null, group_id = 0, line = 0 where model = :model")
        .expect("Error!");

    match maria_conn.exec_drop(
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
