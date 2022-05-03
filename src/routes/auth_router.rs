use crate::db::maria_lib::DataBase;
use base64;
use sha2::{Digest, Sha512};

use actix_web::{
    get, http::header::ContentType, post, web, HttpResponse, HttpResponseBuilder, Responder,
    cookie::Cookie
};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

use serde_json::json;

use crate::routes::functions::auth::*;

#[derive(Serialize)]
struct Obj {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub idx: i32,
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginParam {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(data: web::Json<LoginParam>) -> HttpResponse {
    let mut db = DataBase::init();

    let mut hasher = Sha512::new();
    hasher.update(data.password.as_bytes());

    let result = hasher.finalize();

    let hash_pw = base64::encode(&result);

    let stmt = db
        .conn
        .prep(r"SELECT idx, email, password, name from users where email = :email")
        .expect("stmt error");

    let row: Vec<User> = db
        .conn
        .exec_map(
            stmt,
            params! {
              "email" => &data.email,
            },
            |(idx, email, password, name)| User {
                idx,
                email,
                password,
                name,
            },
        )
        .expect("select Error");

    if row.len() == 0 {
        return HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .body("{ \"code\" : 0}")
    }

    if hash_pw != row[0].password {
        HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .body("{ \"code\" : 0}")
    } else {
        let token: String = issue_jwt(&row[0]);

        let json = json!({
            "code" : 1,
            "token" : token
        });

        let cookie = Cookie::new("token", token);

        HttpResponse::Ok()
            .cookie(cookie)
            .content_type(ContentType::json())
            .body(serde_json::to_string(&json).expect("Error!"))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Register {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[post("/register")]
pub async fn register(data: web::Json<Register>) -> impl Responder {
    let mut db = DataBase::init();

    let mut hasher = Sha512::new();
    hasher.update(data.password.as_bytes());

    let result = hasher.finalize();

    let hash_pw = base64::encode(&result);

    let stmt = db
        .conn
        .prep(r"INSERT INTO users(email, password, name) VALUES (:email, :password, :name)")
        .expect("Error!");

    let mut json = json!({});

    match db.conn.exec_drop(
        stmt,
        params! {
          "email" => &data.email,
          "password" => hash_pw,
          "name" => &data.name
        },
    ) {
        Ok(_) => {
            json["code"] = json!(1);
        }
        Err(_) => {
            json["code"] = json!(0);
            json["description"] = json!("duplication email");
        }
    }

    web::Json(json)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Email {
    pub email: String,
}

#[post("/check")]
pub async fn check_duple(data: web::Json<Email>) -> impl Responder {
    let mut db = DataBase::init();

    let stmt = db
        .conn
        .prep(r"SELECT email from users where email = :email")
        .expect("stmt error");

    let data: Vec<Email> = db
        .conn
        .exec_map(
            stmt,
            params! {
              "email" => &data.email,
            },
            |email| Email { email },
        )
        .expect("Error");

    let i = data.len();

    let json = json!({ "message": i });

    web::Json(json)
}

#[post("/email/key")]
pub async fn send_key(data: web::Json<Email>) -> impl Responder {
    //1. create Code
    let code = create_code();

    //2. save in redis with email, code, time( 3분 초과시 안되게 할것이기 때문)
    save_redis(&data.email, &code);

    //3. 이메일 전송
    match send_mail(&data.email, &code) {
        Ok(_) => {
            let json = json!({ "code": 1 });
            web::Json(json)
        }
        Err(_) => {
            let json = json!({ "code": 0 });

            web::Json(json)
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Verify {
    pub email: String,
    pub code: String,
}

#[post("/email/auth")]
pub async fn email_auth(verify: web::Json<Verify>) -> impl Responder {
    //1. email이 저장되어있는지 확인
    let value = get_redis_email(&verify.email);

    //값이 없으면 0 리턴
    if value == "{}" {
        let json = json!({ "code": 0, "description" : "not exist email value" });
        return web::Json(json);
    } else {
        //값이 있으면 코드 값과, 시간 경과 여부를 확인합니다.
        let json = verify_code(&verify, &value);

        return web::Json(json);
    }
}
