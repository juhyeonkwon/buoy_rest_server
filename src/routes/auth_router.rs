use crate::db::maria_lib::DataBase;
use base64;
use sha2::{Digest, Sha512};

use actix_web::{
    get, http::header::ContentType, post, web, HttpResponse, HttpResponseBuilder, Responder,
};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

use serde_json::json;

#[derive(Serialize)]
struct Obj {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub idx: i32,
    pub id: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginParam {
    pub id: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(data: web::Form<LoginParam>) -> HttpResponse {
    let mut db = DataBase::init();

    let mut hasher = Sha512::new();
    hasher.update(data.password.as_bytes());

    let result = hasher.finalize();

    let hash_pw = base64::encode(&result);

    let stmt = db
        .conn
        .prep(r"SELECT idx, id, password, name from users where id = :id")
        .expect("stmt error");

    let row: Vec<User> = db
        .conn
        .exec_map(
            stmt,
            params! {
              "id" => &data.id,
            },
            |(idx, id, password, name)| User {
                idx,
                id,
                password,
                name,
            },
        )
        .expect("select Error");

    println!("{}, {}", hash_pw, row[0].password);

    if hash_pw != row[0].password {
        HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .body("{ \"code\" : 0}")
    } else {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("{ \"code\" : 1}")
    }
}

#[derive(Deserialize, Serialize)]
pub struct Register {
    pub id: String,
    pub password: String,
    pub name: String,
}

#[post("/register")]
pub async fn register(data: web::Form<Register>) -> impl Responder {
    let mut db = DataBase::init();

    let mut hasher = Sha512::new();
    hasher.update(data.password.as_bytes());

    let result = hasher.finalize();

    let hash_pw = base64::encode(&result);

    let stmt = db
        .conn
        .prep(r"INSERT INTO users(id, password, name) VALUES (:id, :password, :name)")
        .expect("Error!");

    let mut json = json!({});

    match db.conn.exec_drop(
        stmt,
        params! {
          "id" => &data.id,
          "password" => hash_pw,
          "name" => &data.name
        },
    ) {
        Ok(_) => {
            json["code"] = json!(1);
        }
        Err(_) => {
            json["code"] = json!(0);
            json["description"] = json!("duplication id");
        }
    }

    web::Json(json)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Id {
    pub id: String,
}

#[post("/check")]
pub async fn check_duple(data: web::Form<Id>) -> impl Responder {
    let mut db = DataBase::init();

    let stmt = db
        .conn
        .prep(r"SELECT id from users where id = :id")
        .expect("stmt error");

    let data: Vec<Id> = db
        .conn
        .exec_map(
            stmt,
            params! {
              "id" => &data.id,
            },
            |id| Id { id },
        )
        .expect("Error");

    let i = data.len();

    let json = json!({ "message": i });

    web::Json(json)
}
