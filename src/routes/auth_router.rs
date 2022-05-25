use base64;
use sha2::{Digest, Sha512};

use actix_web::{
    cookie::Cookie, get, http::header::ContentType, post, web, HttpResponse, Responder, HttpRequest /*HttpResponseBuilder*/ 
};
use mysql::prelude::*;
use mysql::*;
use serde_json::json;

use crate::routes::functions::auth::*;
use crate::routes::functions::oauth::*;
use crate::db::model::auth_model::*;


use serde::Serialize;
use serde::Deserialize;

use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct GoogleOauth {
//     pub credential: String,
//     pub g_csrf_token: String,
//     pub v-01d69954: String,
// }

pub struct GoogleKey {
    pub n : String,
    pub e : String,
    pub alg : String,
    pub _use : String,
    pub kid : String,
    pub kty : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleParam {
    pub clientId: String,
    pub credential: String,
    pub select_by: String,
}

#[post("/oauth/google")]
pub async fn google(pool: web::Data<Pool>, dtd : web::Json<GoogleParam>) -> impl Responder {

    let google_key = GoogleKey {
        n: "yY0IYTajOWIdeweQB5ZMnvXquuSu2eDDOu1u2uw9_23YMe0nT72o-jBnHL4qG8UuEzYHeE6Smr8h-k75WqRC2aSOlaPFAoef9XYJ8CFFBgDPyWDWAqwmoOZeAIw3a_F6YmBA3CU0NcIYbrgFDVx-ZQmwj7VGUZJUno7MuafMK3lemcHx505j0TPmdrNfIJB3hVwFK7CvNxkRyE9lczm0HSbFnn8JXKxXimHUUqDa3Xh4v58gy2qsyUA8BWafvrrMJ5NdTOWU5gN2Ly7I4WcOT_ny2GsmQvUSdn9--NyZK3pQIPr158y6MFGxLZvYlCN4YqkHITial3WJ73l6HEIxBw".to_owned(),
        e: "AQAB".to_owned(),
        alg: "RS256".to_owned(),
        _use: "sig".to_owned(),
        kid: "b1a8259eb07660ef23781c85b7849bfa0a1c806c".to_owned(),
        kty: "RSA".to_owned()
    };



    // let header = jsonwebtoken::decode_header(&string.as_ref()).unwrap();


    let msg = jsonwebtoken::decode::<serde_json::Value>(&dtd.credential, &DecodingKey::from_rsa_components(&google_key.n, &google_key.e).unwrap(), &Validation::new(Algorithm::RS256)).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let mut value = oauth_login_process(&mut conn, 1, msg.claims["email"].to_string());


    if value.get("code").unwrap() == 1 {
        value["token"] = json!(issue_sns_jwt(&mut conn, msg.claims["email"].to_string()));
    } else if value.get("code").unwrap() == 0 {
        value["email"] = json!(msg.claims.get("email").unwrap());
        value["name"] = json!(msg.claims.get("name").unwrap());
    }

    web::Json(value)
}

#[get("/oauth/naver")]
pub async fn naver(pool: web::Data<Pool>, query : web::Query<Naver>) -> impl Responder {

    let token = get_token_naver(query).await;

    if token["access_token"] == json!(null) {
        let json = json!({ "code": 0 });

        return web::Json(json)
    }

    let json = get_profile_naver(token).await;

    let mut conn = pool.get_conn().unwrap();

    let mut value = oauth_login_process(&mut conn, 2, serde_json::to_string(json.get("response").unwrap().get("email").unwrap()).unwrap());

    if value.get("code").unwrap() == 1 {
        value["token"] = json!(issue_sns_jwt(&mut conn, serde_json::to_string(json.get("response").unwrap().get("email").unwrap()).unwrap()));
    } else if value.get("code").unwrap() == 0 {
        let email = json.get("response").unwrap().get("email").unwrap().to_string();
        value["email"] = json!(&email[1..email.len()-1]);
        value["name"] = json!(json.get("response").unwrap().get("name").unwrap());
    }

    web::Json(value)
}

#[post("/oauth/kakao")]
pub async fn kakao(pool: web::Data<Pool>, query : web::Json<serde_json::Value>) -> impl Responder {

    println!("query {:#?}", query);

    let mut conn = pool.get_conn().unwrap();

    let mut value = oauth_login_process(&mut conn, 3, serde_json::to_string(query.get("kakao_account").unwrap().get("email").unwrap()).unwrap());

    if value.get("code").unwrap() == 1 {
        value["token"] = json!(issue_sns_jwt(&mut conn, serde_json::to_string(query.get("kakao_account").unwrap().get("email").unwrap()).unwrap()));
    } else if value.get("code").unwrap() == 0 {
        let email = query.get("kakao_account").unwrap().get("email").unwrap().to_string();
        let name = query.get("kakao_account").unwrap().get("profile").unwrap().get("nickname").unwrap().to_string();

        value["email"] = json!(&email[1..email.len()-1]);
        value["name"] = json!(&name[1..name.len()-1]);
    }

    web::Json(value)
}

#[post("/oauth/register")]
pub async fn oauth_register(pool: web::Data<Pool>, data: web::Json<OauthRegister>) -> impl Responder {

    //랜덤으로 패스워드 생성 다람쥐
    let password: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();

    let mut maria_conn = pool.get_conn().unwrap();

    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());


    let result = hasher.finalize();

    let hash_pw = base64::encode(&result);

    let stmt = maria_conn
        .prep(r"INSERT INTO users(email, password, name, sns_type) VALUES (:email, :password, :name, :sns_type)")
        .expect("Error!");

    let mut json = json!({});

    match maria_conn.exec_drop(
        stmt,
        params! {
          "email" => &data.email,
          "password" => hash_pw,
          "name" => &data.name,
          "sns_type" => data.sns_type
        },
    ) {
        Ok(_) => {
            json["code"] = json!(1);
        }
        Err(_) => {
            json["code"] = json!(0);
        }
    }

    web::Json(json)
}
