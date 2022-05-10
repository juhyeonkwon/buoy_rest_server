use base64;
use sha2::{Digest, Sha512};

use actix_web::{
    cookie::Cookie, /*get,*/ http::header::ContentType, post, web, HttpResponse, Responder /*HttpResponseBuilder*/ 
};
use mysql::prelude::*;
use mysql::*;
use serde_json::json;

use crate::routes::functions::auth::*;
use crate::db::model::auth_model::*;


#[post("/login")]
pub async fn login(pool: web::Data<Pool>, data: web::Json<LoginParam>) -> HttpResponse {
    let mut maria_conn = pool.get_conn().unwrap();

    let mut hasher = Sha512::new();
    hasher.update(data.password.as_bytes());

    let result = hasher.finalize();

    let hash_pw = base64::encode(&result);

    let stmt = maria_conn
        .prep(r"SELECT idx, email, password, name, admin from users where email = :email")
        .expect("stmt error");

    let row: Vec<User> = maria_conn
        .exec_map(
            stmt,
            params! {
              "email" => &data.email,
            },
            |(idx, email, password, name, admin)| User {
                idx,
                email,
                password,
                name,
                admin,
            },
        )
        .expect("select Error");

    if row.len() == 0 {
        return HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .body("{ \"code\" : 0}");
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

#[post("/register")]
pub async fn register(pool: web::Data<Pool>, data: web::Json<Register>) -> impl Responder {
    let mut maria_conn = pool.get_conn().unwrap();

    let mut hasher = Sha512::new();
    hasher.update(data.password.as_bytes());

    let result = hasher.finalize();

    let hash_pw = base64::encode(&result);

    let stmt = maria_conn
        .prep(r"INSERT INTO users(email, password, name) VALUES (:email, :password, :name)")
        .expect("Error!");

    let mut json = json!({});

    match maria_conn.exec_drop(
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


#[post("/check")]
pub async fn check_duple(pool: web::Data<Pool>, data: web::Json<Email>) -> impl Responder {
    let mut maria_conn = pool.get_conn().unwrap();

    let stmt = maria_conn
        .prep(r"SELECT email from users where email = :email")
        .expect("stmt error");

    let data: Vec<Email> = maria_conn
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
pub async fn send_key(redis : web::Data<redis::Client>, data: web::Json<Email>) -> impl Responder {

    let mut redis_conn = redis.get_connection().unwrap();
   
   
    //1. create Code
    let code = create_code();
    //2. save in redis with email, code, time( 3분 초과시 안되게 할것이기 때문)
    save_redis(&data.email, &code, &mut redis_conn);

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

#[post("/email/auth")]
pub async fn email_auth(redis : web::Data<redis::Client>, verify: web::Json<Verify>) -> impl Responder {
    let mut redis_conn = redis.get_connection().unwrap();

    //1. email이 저장되어있는지 확인
    let value = get_redis_email(&verify.email, &mut redis_conn);

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
