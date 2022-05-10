use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use std::env;

use chrono;
use chrono::prelude::*;
use chrono::Duration;

use crate::custom_middleware::jwt::Claims;
use crate::db::model::auth_model::{User, Verify};

use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use rand::prelude::*;
use redis::Commands;

use base64;
use sha2::{Digest, Sha512};

use serde_json::{json, Value};

pub fn issue_jwt(user: &User) -> String {
    let exp: DateTime<Local> = Local::now() + Duration::days(365 * 10);

    let timestamp = exp.timestamp_millis();

    let secret: String = match env::var("SECRET") {
        Ok(v) => v,
        Err(_) => panic!("Env SECRET Not Found!"),
    };

    let claim = Claims {
        idx: user.idx,
        email: user.email.to_owned(),
        admin: user.admin,
        exp: timestamp as usize,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("error!")
}

pub fn create_code() -> String {
    let mut rng = rand::thread_rng();

    let int_code: i32 = rng.gen_range(100000..999999);

    int_code.to_string()
}

pub fn save_redis(email: &String, code: &String, redis_conn : &mut redis::Connection) {

    //3분의 제한시간을 줍니다람쥐!!
    let exp: DateTime<Local> = Local::now() + Duration::minutes(3);

    let timestamp = exp.timestamp_millis();

    let json = json!({
      "code" : code,
      "exp" : timestamp
    });

    let _: () = redis::cmd("SET")
        .arg(email)
        .arg(serde_json::to_string(&json).expect("parse Error!"))
        .query(redis_conn)
        .expect("Error!");
}

pub fn send_mail(email_to: &String, code: &String) -> Result<i16, i16> {
    let id: String = match env::var("DXDATA_ID") {
        Ok(v) => v,
        Err(_) => panic!("Env DXDATA_ID Not Found!"),
    };

    let pw: String = match env::var("DXDATA_PW") {
        Ok(v) => v,
        Err(_) => panic!("Env DXDATA_PW Not Found!"),
    };

    let html = String::from("
                  <h2>
                    본인확인 인증코드입니다. 
                  </h2>
                  <br /><br />
                  본인확인을 위해 아래의 인증코드를 화면에 입력해주세요<br />
                  <br />
                  <h1 style=\"background: #eeeeee; height: 50px; display:flex; align-items: center;justify-content: center;\" >
                        ") + code + "
                  </h1>
                  <br />
                  <br />
                  감사합니다.";

    let email = Message::builder()
        .from("no_reply <no_reply@dxdata.co.kr>".parse().unwrap())
        .to(email_to.parse().unwrap())
        .subject("[dxdata] 본인 인증을 위한 인증코드 안내메일입니다.")
        .multipart(MultiPart::alternative_plain_html(String::from(""), html))
        .unwrap();

    let creds = Credentials::new(id, pw);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("outbound.daouoffice.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(1),
        Err(e) => {
            println!("{:#?}", e);
            Err(-1)
        }
    }
}

pub fn get_redis_email(email: &String, redis_conn : &mut redis::Connection) -> String {

    match redis::cmd("GET").arg(email).query(redis_conn) {
        Ok(v) => v,
        Err(_) => String::from("{}"),
    }
}

#[derive(Deserialize, Serialize)]
struct RedisCode {
    pub code: String,
    pub exp: i64,
}

pub fn verify_code(input_data: &Verify, redis_val: &String) -> Value {
    //code 값을 RedisCode 값으로 변환
    let value: RedisCode = serde_json::from_str(redis_val).expect("Error!");

    //현재 시간을 구함
    let now: DateTime<Local> = Local::now();

    let timestamp = now.timestamp_millis();

    //코드가 같을 경우 시간을 비교해봅니다.
    if input_data.code == value.code {
        //만약 시간이 지났을 경우입니다.
        if timestamp > value.exp {
            json!({ "code": 0, "description" : "timeout" })
        } else {
            //시간이 지나지 않았다면 성공 메세지를 보여줍니다.
            json!({ "code": 1, "description" : "success" })
        }
    } else {
        //코드가 맞지 않으면 오류를 냅니다.
        json!({ "code": 0, "description" : "not matched" })
    }
}

// hash패스워드로 변환
pub fn get_hash(word: &String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(word.as_bytes());

    let result = hasher.finalize();

    let hash = base64::encode(&result);

    hash
}
