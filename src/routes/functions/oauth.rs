use std::env;

use actix_web::web;

use crate::db::model::auth_model::*;

use mysql::prelude::*;
use mysql::*;

use serde_json::json;
use serde::{Deserialize, Serialize};

use jsonwebtoken::{encode, EncodingKey, Header};

use crate::custom_middleware::jwt::Claims;

use chrono;
use chrono::prelude::*;
use chrono::Duration;

pub async fn get_token_naver(query : web::Query<Naver>) -> serde_json::Value {
  let secret = env::var("CLIENT_SECRET").expect("ENV NOT FOUND");


  let url = String::from("https://nid.naver.com/oauth2.0/token?grant_type=authorization_code&client_id=v5hITj0N_18gCcJFIECV&client_secret=") + &secret + "&code=" + &query.code + "&state=" + &query.state;

  let body = reqwest::get(url).await.unwrap().text().await.unwrap();

  let json : serde_json::Value = serde_json::from_str(&body).unwrap();

  json
}

pub async fn get_profile_naver(token : serde_json::Value) -> serde_json::Value {

  let client = reqwest::Client::new();

  let access_token : String =  "Bearer ".to_owned() + &token["access_token"].to_string();

  println!("{}", access_token);

  let body = client.get("https://openapi.naver.com/v1/nid/me").header("Authorization", access_token).send().await.unwrap().text().await.unwrap();

  let json : serde_json::Value = serde_json::from_str(&body).unwrap();

  json

}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthUser {
    pub idx: i32,
    pub email: String,
    pub sns_type : i8,
    pub name : Option<String>,
    pub sns_token : Option<String>,
}



//이메일이 있는데 sns 타입이랑 같으면 회원가입 되어있는것, 다르면 다른 sns 계정이나 아이디가 있는것, 없으면 회원가입을 진행
pub fn oauth_login_process(conn : &mut PooledConn, sns : i8, email : String) -> serde_json::Value {
  
  let prep = conn.prep("SELECT a.idx, a.email, a.sns_type, b.name AS sns_name, a.sns_token FROM users a, sns b WHERE email = :email AND a.sns_type = b.sns_idx").unwrap();


  let res = conn.exec_map(prep, params!{"email" => &email[1..email.len()-1]}, |(idx,
                                email,
                                sns_type,
                                name,
                                sns_token)| AuthUser {
                                idx,
                                email,
                                sns_type,
                                name,
                                sns_token,
                            }).unwrap();


  if res.len() == 0 {
    json!({
      "code" : 0,
      "message" : "not found"
    })
  } else if sns == res[0].sns_type {
    return json!({
      "code" : 1,
      "message" : "matched",
    })
  } else {
    if res[0].sns_type == 0 {
      return json!({
        "code" : 3,
        "message" : "not a SNS Account",
        "sns_type" : res[0].sns_type
      })
    } else {
      return json!({
        "code" : 2,
        "message" : "another acount already exist",
        "sns_type" : res[0].sns_type,
        "sns" : res[0].name
      })
    }
  }
}



pub fn issue_sns_jwt(maria_conn : &mut PooledConn, email: String) -> String {
  let exp: DateTime<Local> = Local::now() + Duration::days(365 * 10);

  let timestamp = exp.timestamp_millis();

  let secret: String = match env::var("SECRET") {
      Ok(v) => v,
      Err(_) => panic!("Env SECRET Not Found!"),
  };

  let stmt = maria_conn
  .prep(r"SELECT idx, email, password, name, admin from users where email = :email")
  .expect("stmt error");

  let user: Vec<User> = maria_conn
    .exec_map(
        stmt,
        params! {
          "email" => &email[1..email.len()-1],
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


  let claim = Claims {
      idx: user[0].idx,
      email: user[0].email.to_owned(),
      admin: user[0].admin,
      exp: timestamp as usize,
  };

  encode(
      &Header::default(),
      &claim,
      &EncodingKey::from_secret(secret.as_ref()),
  )
  .expect("error!")
}
