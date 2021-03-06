// #![allow(non_snake_case)]

// #[cfg(test)]
// mod tests {

//     use serde::{Deserialize, Serialize};
//     use serde_json::{json, Value};
//     use std::f64::consts::PI;

//     use std::env;

//     use chrono;
//     use chrono::prelude::*;
//     use chrono::Duration;

//     #[test]
//     fn date_test() {
//         let now: DateTime<Local> = Local::now();

//         let now_str = now.to_string();

//         let ab = format!("{}{}{}", &now_str[0..4], &now_str[5..7], &now_str[8..10]);
//         let cd = format!("{}{}", &now_str[11..14], &now_str[14..16]);

//         println!("{:?}{}", ab, cd);
//     }

//     const RE: f64 = 6371.00877; // 지구 반경(km)
//     const GRID: f64 = 5.0; // 격자 간격(km)
//     const SLAT1: f64 = 30.0; // 투영 위도1(degree)
//     const SLAT2: f64 = 60.0; // 투영 위도2(degree)
//     const OLON: f64 = 126.0; // 기준점 경도(degree)
//     const OLAT: f64 = 38.0; // 기준점 위도(degree)
//     const XO: f64 = 43.0; // 기준점 X좌표(GRID)
//     const YO: f64 = 136.0; // 기준점 Y좌표(GRID)

//     const DEGRAD: f64 = PI / 180.0;
//     // const RADDEG : f64 = 180.0 / PI;

//     #[test]
//     fn dfs_xy_conv_test() {
//         let v1: f64 = 34.7973052;
//         let v2: f64 = 128.4642589;

//         let re = RE / GRID;
//         let slat1 = SLAT1 * DEGRAD;
//         let slat2 = SLAT2 * DEGRAD;
//         let olon = OLON * DEGRAD;
//         let olat = OLAT * DEGRAD;

//         let mut sn: f64 = (PI * 0.25 + slat2 * 0.5).tan() / (PI * 0.25 + slat1 * 0.5).tan();

//         sn = (slat1.cos() / slat2.cos()).ln() / sn.ln();

//         let mut sf = (PI * 0.25 + slat1 * 0.5).tan();

//         sf = (sf.powf(sn) * slat1.cos()) / sn;

//         let mut ro = (PI * 0.25 + olat * 0.5).tan();
//         ro = (re * sf) / ro.powf(sn);
//         let mut rs = serde_json::json!({
//             "lat" : 0.0,
//             "lng" : 0.0
//         });

//         rs["lat"] = json!(v1);
//         rs["lng"] = json!(v2);
//         let mut ra = (PI * 0.25 + v1 * DEGRAD * 0.5).tan();
//         ra = (re * sf) / ra.powf(sn);
//         let mut theta = v2 * DEGRAD - olon;

//         if theta > PI {
//             theta -= 2.0 * PI
//         };

//         if theta < -PI {
//             theta += 2.0 * PI
//         };

//         theta *= sn;

//         rs["x"] = json!((ra * theta.sin() + XO + 0.5).floor());
//         rs["y"] = json!((ro - ra * theta.cos() + YO + 0.5).floor());

//         println!("{:#?}", rs);
//     }

//     #[derive(Debug, Deserialize, Serialize)]
//     struct FcstInfo {
//         #[allow(non_snake_case)]
//         baseDate: String,
//         #[allow(non_snake_case)]
//         baseTime: String,
//         category: String,
//         nx: i16,
//         ny: i16,
//         #[allow(non_snake_case)]
//         obsrValue: String,
//     }

//     // #[test]
//     // fn request_weather() {
//     //     let url = "https://apis.data.go.kr/1360000/VilageFcstInfoService_2.0/getUltraSrtNcst?serviceKey=f07kNaBvNTS%2FVHWxNplYgJJpu%2B75KQARZURTpNtwE7PAjA0hFZfmY6k9iX3QDVB2ux6%2BMulcWogEeXF5OSWIHQ%3D%3D&pageNo=1&numOfRows=1000&dataType=JSON&base_date=20220418&base_time=0800&nx=55&ny=127";

//     //     let resp = reqwest::blocking::get(url)
//     //         .expect("Error!")
//     //         .text()
//     //         .expect("Error!");

//     //     let mut temp: Value = serde_json::from_str(&resp).expect("Error!");
//     //     let a: Vec<FcstInfo> =
//     //         serde_json::from_value(temp["response"]["body"]["items"]["item"].take())
//     //             .expect("Error!");

//     //     println!("{:#?}", a);
//     // }

//     use dotenv::dotenv;

//     use crate::db::maria_lib::DataBase;
//     use crate::db::model::detail_model::GroupLineAvg;
//     use crate::db::redis_lib::connect_redis;
//     use crate::routes::functions::detail_data::{
//         get_group_detail_data, get_group_line_data, get_line_history,
//     };
//     use mysql::prelude::*;
//     use mysql::*;

//     use crate::db::model::detail_model::List;

//     //crypto 테스트
//     use base64;
//     use sha2::{Digest, Sha512};

//     #[test]
//     fn crypto_test() {
//         let mut hasher = Sha512::new();
//         hasher.update(b"test");
//         let result = hasher.finalize();
//         let encoded = base64::encode(&result);

//         println!("Binary hash: {:?}", encoded)
//     }

//     use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

//     /// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
//     #[derive(Debug, Serialize, Deserialize)]
//     struct Claims {
//         idx: i16,
//         id: String,
//         exp: usize,
//     }

//     #[test]
//     fn jwt_test() {
//         dotenv().ok();
//         let now2: DateTime<Local> = Local::now();

//         let now: DateTime<Local> = Local::now() - Duration::days(1);

//         let timestamp = now.timestamp_millis();
//         let timestam2 = now2.timestamp_millis();

//         let secret: String = match env::var("SECRET") {
//             Ok(v) => v,
//             Err(_) => panic!("Env SECRET Not Found!"),
//         };

//         let claim = Claims {
//             idx: 1,
//             id: "test".to_owned(),
//             exp: timestamp as usize,
//         };

//         let token = encode(
//             &Header::default(),
//             &claim,
//             &EncodingKey::from_secret(secret.as_ref()),
//         )
//         .expect("error!");

//         println!("{}", token);

//         let mut val = Validation::new(Algorithm::HS256);

//         let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &val)
//             .expect("Error!");

//         if decoded.claims.exp < timestam2 as usize {
//             println!("늦엇사");
//         }

//         println!("{}, {}", decoded.claims.exp, timestam2);
//     }
// }
