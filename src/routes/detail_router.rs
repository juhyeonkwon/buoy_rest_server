use crate::db::maria_lib::DataBase;
use crate::db::model::common_model::GroupList;
use crate::db::model::detail_model::{
    AssignedBuoy, BuoyAllocate, BuoyQuery, GroupAdd, GroupId, GroupModify, Obj, UnassignedBuoy,
};
use crate::db::redis_lib::connect_redis;

use actix_web::error::ErrorUnauthorized;
use actix_web::{
    delete, get, post, put, web, web::ReqData, /*HttpResponse, post,*/ Responder, Result,
};
use mysql::prelude::*;
use mysql::*;
use serde_json::json;

use crate::routes::functions::detail_data::{
    check_owned, check_owned_buoy, get_buoy, get_buoy_history, get_buoy_list, get_group_data,
    get_group_detail_data, get_group_history,
};

use crate::custom_middleware::jwt::Claims;

#[get("/group/list")]
pub async fn group_list(token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    let stmt = db
        .conn
        .prep("SELECT group_id, group_name FROM buoy_group where group_id > 0 AND user_idx = :idx")
        .expect("PREP ERROR");

    let row: Vec<GroupList> = db
        .conn
        .exec_map(
            stmt,
            params! {"idx" => user.idx},
            |(group_id, group_name)| GroupList {
                group_id,
                group_name,
            },
        )
        .expect("select Error");

    web::Json(row)
}

#[get("/group")]
pub async fn group_detail(
    token: ReqData<Claims>,
    query: web::Query<GroupId>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    if check_owned(&mut db, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let val = get_group_detail_data(query.group_id, user.idx, &mut db);

    Ok(web::Json(val))
}

#[get("/group/web")]
pub async fn group_detail_web(
    token: ReqData<Claims>,
    query: web::Query<GroupId>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    if check_owned(&mut db, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let mut group_data = get_group_data(&mut db, query.group_id, user.idx);
    let val = get_group_detail_data(query.group_id, user.idx, &mut db);

    group_data["lines"] = json!(val);

    Ok(web::Json(group_data))
}

#[get("/group/history")]
pub async fn group_history(
    token: ReqData<Claims>,
    query: web::Query<GroupId>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    if check_owned(&mut db, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let mut conn = connect_redis();
    let val = get_group_history(query.group_id, &mut conn);

    Ok(web::Json(val))
}

// 0 일반, 1 연승, 2 땟목, 3 기타
#[put("/group/modify")]
pub async fn group_modify(
    token: ReqData<Claims>,
    param: web::Json<GroupModify>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    if check_owned(&mut db, param.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let stmt = db
        .conn
        .prep(
            "UPDATE buoy_group 
                             SET
                                 group_name = :group_name,
                                 group_system = :group_system,
                                 plain_buoy = :plain_buoy
                             WHERE group_id = :group_id",
        )
        .expect("Error!");

    match db.conn.exec_drop(
        stmt,
        params! {
            "group_name" => param.group_name.to_owned(),
            "group_system" => param.group_system,
            "plain_buoy" => param.plain_buoy,
            "group_id" => param.group_id
        },
    ) {
        Ok(_) => {
            let json = json!({"code" : 1});
            Ok(web::Json(json))
        }
        Err(e) => {
            println!("{:#?}", e);
            let json = json!({"code" : 0});
            Ok(web::Json(json))
        }
    }
}

#[post("/group/create")]
pub async fn create_group(
    token: ReqData<Claims>,
    data: web::Json<GroupAdd>,
) -> Result<impl Responder> {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    let stmt = db.conn.prep("INSERT INTO buoy_group(group_name, group_system, plain_buoy, user_idx) VALUES (:group_name, :group_system, :plain_buoy, :user_idx)").expect("PREP ERROR");

    match db.conn.exec_drop(
        stmt,
        params! {
            "group_name" => &data.group_name,
            "group_system" => data.group_system,
            "plain_buoy" => data.plain_buoy,
            "user_idx" => user.idx
        },
    ) {
        Ok(_) => {
            let json = json!({"code" : 1});
            Ok(web::Json(json))
        }
        Err(e) => {
            println!("{:#?}", e);
            let json = json!({"code" : 0});
            Ok(web::Json(json))
        }
    }
}

#[delete("/group/delete")]
pub async fn delete_group(
    token: ReqData<Claims>,
    data: web::Json<GroupId>,
) -> Result<impl Responder> {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if check_owned(&mut db, data.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let stmt = db
        .conn
        .prep("UPDATE buoy_model set group_id = 0, line = 0 WHERE group_id = :group_id ")
        .expect("PREP ERR");

    match db.conn.exec_drop(
        stmt,
        params! {
            "group_id" => data.group_id,
        },
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("{:#?}", e);
        }
    }

    let stmt2 = db
        .conn
        .prep("DELETE FROM buoy_group where group_id = :group_id")
        .expect("PREP ERR");

    match db.conn.exec_drop(
        stmt2,
        params! {
            "group_id" => data.group_id,
        },
    ) {
        Ok(_) => {
            let json = json!({"code" : 1});
            Ok(web::Json(json))
        }
        Err(e) => {
            println!("{:#?}", e);
            let json = json!({"code" : 0});
            Ok(web::Json(json))
        }
    }
}

#[get("/buoy/list")]
pub async fn buoy_group_list(
    token: ReqData<Claims>,
    query: web::Query<GroupId>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    if check_owned(&mut db, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let val = get_buoy_list(query.group_id, &mut db);

    Ok(web::Json(val))
}

#[get("/buoy")]
pub async fn buoy_spec(
    token: ReqData<Claims>,
    query: web::Query<BuoyQuery>,
) -> Result<impl Responder> {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut db, &query.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"));
    }

    let val = get_buoy(&query.model, &mut db);

    Ok(web::Json(val))
}

#[get("/buoy/history")]
pub async fn buoy_detail(
    token: ReqData<Claims>,
    query: web::Query<BuoyQuery>,
) -> Result<impl Responder> {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut db, &query.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"));
    }

    let val = get_buoy_history(&query.model);

    Ok(web::Json(val))
}

#[get("/buoy/assigned")]
pub async fn buoy_assigned(token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    let stmt = db
        .conn
        .prep(
            "SELECT model_idx,
                                    model,
                                    latitude,
                                    longitude,
                                    a.group_id,
                                    b.group_name
                                    FROM 
                                        buoy_model a, buoy_group b
                                    WHERE a.group_id = b.group_id 
                                        AND a.user_idx = :user_idx 
                                        AND a.group_id > 0",
        )
        .expect("Error!");

    let value: Vec<AssignedBuoy> = db
        .conn
        .exec_map(
            stmt,
            params! {
                "user_idx" => user.idx,
            },
            |(model_idx, model, latitude, longitude, group_id, group_name)| AssignedBuoy {
                model_idx,
                model,
                latitude,
                longitude,
                group_id,
                group_name,
            },
        )
        .expect("db Error!");

    web::Json(value)
}

#[get("/buoy/unassigned")]
pub async fn buoy_unassigned(token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    let mut db = DataBase::init();

    let stmt = db
        .conn
        .prep(
            "SELECT model_idx,
                                    model,
                                    latitude,
                                    longitude
                                    FROM 
                                        buoy_model 
                                    WHERE user_idx = :user_idx AND group_id = 0",
        )
        .expect("Error!");

    let value: Vec<UnassignedBuoy> = db
        .conn
        .exec_map(
            stmt,
            params! {
                "user_idx" => user.idx,
            },
            |(model_idx, model, latitude, longitude)| UnassignedBuoy {
                model_idx,
                model,
                latitude,
                longitude,
            },
        )
        .expect("db Error!");

    web::Json(value)
}

#[put("/buoy/allocate")]
pub async fn buoy_allocate(
    token: ReqData<Claims>,
    buoy: web::Json<BuoyAllocate>,
) -> impl Responder {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut db, &buoy.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"));
    }

    if check_owned(&mut db, buoy.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
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
        return Err(ErrorUnauthorized("Not Owned Buoy"));
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
