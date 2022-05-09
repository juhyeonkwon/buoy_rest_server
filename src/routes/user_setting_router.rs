use crate::db::maria_lib::DataBase;

use actix_web::{
    delete, get, post, put, web, web::ReqData, /*HttpResponse, post,*/ Responder, Result,
};

use crate::routes::functions::auth::*;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

use crate::custom_middleware::jwt::Claims;
use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct ModifyPW {
    pub password: String,
    pub new_password: String,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub idx: i32,
    pub email: String,
    pub password: String,
    pub name: String,
}

#[put("/user/password")]
pub async fn modify(token: ReqData<Claims>, data: web::Json<ModifyPW>) -> Result<impl Responder> {
    let mut db = DataBase::init();

    let user: Claims = token.into_inner();

    let hash_pw = get_hash(&data.password);

    let stmt = db
        .conn
        .prep(r"SELECT idx, email, password, name from users where idx = :user_idx")
        .expect("stmt error");

    let row: Vec<User> = db
        .conn
        .exec_map(
            stmt,
            params! {
              "user_idx" => user.idx,
            },
            |(idx, email, password, name)| User {
                idx,
                email,
                password,
                name,
            },
        )
        .expect("select Error");

    if hash_pw != row[0].password {
        let json = json!({"code" : 0, "message" : "password not match"});
        Ok(web::Json(json))
    } else {
        let hash_pw = get_hash(&data.new_password);

        let stmt = db
            .conn
            .prep("UPDATE users set password = :password WHERE idx = :user_idx")
            .expect("Error!!");

        match db.conn.exec_drop(
            stmt,
            params! {
              "password" => hash_pw.to_owned(),
              "user_idx" => user.idx
            },
        ) {
            Ok(_) => {
                let json = json!({"code" : 1});
                return Ok(web::Json(json));
            }
            Err(_) => {
                let json = json!({"code" : 0});
                return Ok(web::Json(json));
            }
        }
    }
}
