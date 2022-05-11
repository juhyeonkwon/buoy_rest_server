use crate::db::model::common_model::GroupList;
use crate::db::model::detail_model::{
    AssignedBuoy, BuoyAllocate, BuoyQuery, GroupAdd, GroupId, GroupModify, UnassignedBuoy,
};

use actix_web::{
    delete, get, post, put, web, web::ReqData, /*HttpResponse, post,*/ Responder, Result, error::ErrorUnauthorized
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
pub async fn group_list(pool: web::Data<Pool>, token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = 
        maria_conn
        .prep("SELECT group_id, group_name FROM buoy_group where group_id > 0 AND user_idx = :idx")
        .expect("PREP ERROR");

    let row: Vec<GroupList> = 
     maria_conn
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
    pool: web::Data<Pool>,
    redis : web::Data<redis::Client>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();
    let mut redis_conn = redis.get_connection().unwrap();
    if check_owned(&mut maria_conn, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let val = get_group_detail_data(query.group_id, user.idx, &mut maria_conn, &mut redis_conn);

    if val.len() == 0 {
        println!("0이다옹");
    }
    
    Ok(web::Json(val))
}

#[get("/group/web")]
pub async fn group_detail_web(
    token: ReqData<Claims>,
    query: web::Query<GroupId>,
    pool : web::Data<mysql::Pool>, 
    redis : web::Data<redis::Client>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();
    let mut redis_conn = redis.get_connection().unwrap();

    if check_owned(&mut maria_conn, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let mut group_data = get_group_data(&mut maria_conn, query.group_id, user.idx);
    let val = get_group_detail_data(query.group_id, user.idx, &mut maria_conn, &mut redis_conn);

    group_data["lines"] = json!(val);

    Ok(web::Json(group_data))
}

#[get("/group/history")]
pub async fn group_history(
    token: ReqData<Claims>,
    query: web::Query<GroupId>,
    pool: web::Data<Pool>,
    redis : web::Data<redis::Client>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();
    let mut redis_conn = redis.get_connection().unwrap();

    if check_owned(&mut maria_conn, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let val = get_group_history(query.group_id, &mut redis_conn);

    Ok(web::Json(val))
}

// 0 일반, 1 연승, 2 땟목, 3 기타
#[put("/group/modify")]
pub async fn group_modify(
    token: ReqData<Claims>,
    param: web::Json<GroupModify>,
    pool: web::Data<Pool>,
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();

    if check_owned(&mut maria_conn, param.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let stmt = 
    maria_conn
        .prep(
            "UPDATE buoy_group 
                             SET
                                 group_name = :group_name,
                                 group_system = :group_system,
                                 plain_buoy = :plain_buoy
                             WHERE group_id = :group_id",
        )
        .expect("Error!");

    match maria_conn.exec_drop(
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
    pool: web::Data<Pool>,
) -> Result<impl Responder> {

    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = maria_conn.prep("INSERT INTO buoy_group(group_name, group_system, plain_buoy, user_idx) VALUES (:group_name, :group_system, :plain_buoy, :user_idx)").expect("PREP ERROR");

    match maria_conn.exec_drop(
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

#[post("/group/delete")]
pub async fn delete_group(
    token: ReqData<Claims>,
    data: web::Json<GroupId>,
    pool: web::Data<Pool>,
) -> Result<impl Responder> {
    
    let mut maria_conn = pool.get_conn().unwrap();

    let user: Claims = token.into_inner();

    if check_owned(&mut maria_conn, data.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let stmt = 
        maria_conn
        .prep("UPDATE buoy_model set group_id = 0, line = 0 WHERE group_id = :group_id ")
        .expect("PREP ERR");

    match maria_conn.exec_drop(
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

    let stmt2 = 
        maria_conn
        .prep("DELETE FROM buoy_group where group_id = :group_id")
        .expect("PREP ERR");

    match maria_conn.exec_drop(
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
    pool: web::Data<Pool>
) -> Result<impl Responder> {
    let user: Claims = token.into_inner();

    
    let mut maria_conn = pool.get_conn().unwrap();

    if check_owned(&mut maria_conn, query.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let val = get_buoy_list(query.group_id, &mut maria_conn);

    Ok(web::Json(val))
}

#[get("/buoy")]
pub async fn buoy_spec(
    token: ReqData<Claims>,
    query: web::Query<BuoyQuery>,
    pool: web::Data<Pool>,
) -> Result<impl Responder> {

    let mut maria_conn = pool.get_conn().unwrap();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut maria_conn, &query.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"));
    }

    let val = get_buoy(&query.model, &mut maria_conn);

    Ok(web::Json(val))
}

#[get("/buoy/history")]
pub async fn buoy_detail(
    token: ReqData<Claims>,
    query: web::Query<BuoyQuery>,
    pool: web::Data<Pool>,
    redis : web::Data<redis::Client>,
) -> Result<impl Responder> {

    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();
    let mut redis_conn = redis.get_connection().unwrap();

    if check_owned_buoy(&mut maria_conn, &query.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"));
    }

    let val = get_buoy_history(&query.model, &mut redis_conn);

    Ok(web::Json(val))
}

#[get("/buoy/assigned")]
pub async fn buoy_assigned(pool: web::Data<Pool>, token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = 
        maria_conn
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

    let value: Vec<AssignedBuoy> = 
        maria_conn
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
pub async fn buoy_unassigned(pool: web::Data<Pool>, token: ReqData<Claims>) -> impl Responder {
    let user: Claims = token.into_inner();

    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = 
        maria_conn
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

    let value: Vec<UnassignedBuoy> = 
        maria_conn
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
    pool: web::Data<Pool>,
    token: ReqData<Claims>,
    buoy: web::Json<BuoyAllocate>,
) -> impl Responder {
    let mut maria_conn = pool.get_conn().unwrap();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut maria_conn, &buoy.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"));
    }

    if check_owned(&mut maria_conn, buoy.group_id, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Group"));
    }

    let stmt = maria_conn
        .prep(
            "UPDATE buoy_model 
             SET 
                group_id = :group_id, 
                line = :line 
             WHERE 
                model = :model",
        )
        .expect("Error!");

    match maria_conn.exec_drop(
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
pub async fn buoy_deallocate(pool: web::Data<Pool>, token: ReqData<Claims>, buoy: web::Json<BuoyQuery>) -> impl Responder {
    let mut maria_conn = pool.get_conn().unwrap();

    let user: Claims = token.into_inner();

    if check_owned_buoy(&mut maria_conn, &buoy.model, user.idx) == 0 {
        return Err(ErrorUnauthorized("Not Owned Buoy"));
    }

    let stmt = maria_conn
        .prep("UPDATE buoy_model set group_id = 0, line = 0 where model = :model")
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
