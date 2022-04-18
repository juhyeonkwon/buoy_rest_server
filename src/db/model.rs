#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::f64::consts::PI;

use crate::db::maria_lib::DataBase;
use mysql::prelude::*;
use mysql::*;

use chrono;
use chrono::prelude::*;
use chrono::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Buoy {
    pub time: String,
    pub model: String,
    pub lat: f64,
    pub lon: f64,
    pub w_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
}

#[derive(Debug)]
pub struct Insertbuoy {
    pub buoy: Buoy,
    pub group_id: i32,
}

#[derive(Serialize, Debug)]
pub struct Modelinfo {
    pub model: String,
    pub group_id: i32,
    pub line: i32,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Serialize, Debug)]
pub struct Group {
    pub group_id: i32,
    pub group_name: String,
    pub group_latitude: f32,
    pub group_longitude: f32,
    pub group_water_temp: f32,
    pub group_salinity: f32,
    pub group_height: f32,
    pub group_weight: f32,
}

#[derive(Serialize, Debug)]
pub struct GroupList {
    pub group_id: i32,
    pub group_name: String,
}

//Get Data Struct

//부이 관측소에서 얻은 tide_velocity 값들
#[derive(Serialize, Deserialize, Debug)]
pub struct TideBuoy {
    pub obs_time: String,
    pub current_direct: String,
    pub current_speed: String,
}

//레이더에서 얻은 tide_velocity 값들임
#[derive(Serialize, Deserialize, Debug)]
pub struct TideRader {
    pub lat: String,
    pub lon: String,
    pub current_direct: String,
    pub current_speed: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TideRaderList {
    pub data: Vec<TideRader>,
}

//기상청 데이터 struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Meteorological {
    time: Time,
    location: LocationDfs,
    data: Option<Vec<FcstInfo>>,
    region: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Time {
    pub date: String,
    pub time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationDfs {
    pub lat: f64,
    pub lng: f64,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct FcstInfo {
    #[allow(non_snake_case)]
    pub baseDate: String,
    #[allow(non_snake_case)]
    pub baseTime: String,
    pub category: String,
    pub nx: i16,
    pub ny: i16,
    #[allow(non_snake_case)]
    pub obsrValue: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Region {
    pub location1: String,
    pub location2: String,
}

#[allow(dead_code)]
const RE: f64 = 6371.00877; // 지구 반경(km)
#[allow(dead_code)]
const GRID: f64 = 5.0; // 격자 간격(km)
#[allow(dead_code)]
const SLAT1: f64 = 30.0; // 투영 위도1(degree)
#[allow(dead_code)]
const SLAT2: f64 = 60.0; // 투영 위도2(degree)
#[allow(dead_code)]
const OLON: f64 = 126.0; // 기준점 경도(degree)
#[allow(dead_code)]
const OLAT: f64 = 38.0; // 기준점 위도(degree)
#[allow(dead_code)]
const XO: f64 = 43.0; // 기준점 X좌표(GRID)
#[allow(dead_code)]
const YO: f64 = 136.0; // 기준점 Y좌표(GRID)

#[allow(dead_code)]
const DEGRAD: f64 = PI / 180.0;

#[allow(dead_code)]
impl Meteorological {
    pub fn init(db : &mut DataBase, v1: &f64, v2: &f64) -> Meteorological {
        let time = Meteorological::get_time();
        let location = Meteorological::dfs_xy_conv(v1, v2);

        let mut temp = Meteorological {
            time,
            location,
            data: None,
            region: String::from(""),
        };

        temp.set_region(db);
        temp.request();
        
        temp
    }

    pub fn request(&mut self) {
        let _key: String = match env::var("GEO_KEY") {
            Ok(v) => v,
            Err(_) => panic!("Env GEO_KEY Not Found!"),
        };

        let url_base =
            "https://apis.data.go.kr/1360000/VilageFcstInfoService_2.0/getUltraSrtNcst?serviceKey=";

        let url = String::from(url_base)
            + &_key
            + "&pageNo=1&numOfRows=1000&dataType=JSON&base_date="
            + &self.time.date
            + "&base_time="
            + &self.time.time
            + "&nx="
            + &self.location.x.to_string()
            + "&ny="
            + &self.location.y.to_string();

        let resp = reqwest::blocking::get(url)
            .expect("Error!")
            .text()
            .expect("Error!");

        let mut temp: Value = serde_json::from_str(&resp).expect("Error!");
        let data: Vec<FcstInfo> =
            serde_json::from_value(temp["response"]["body"]["items"]["item"].take())
                .expect("Error!");

        self.data = Some(data);
    }

    pub fn get_time() -> Time {
        let now: DateTime<Local> = Local::now() - Duration::hours(1);

        let now_str = now.to_string();

        let ab = format!("{}{}{}", &now_str[0..4], &now_str[5..7], &now_str[8..10]);
        let cd = format!("{}{}", &now_str[11..13], &now_str[14..16]);

        Time {
            date: ab,
            time: cd,
        }
    }

    pub fn set_region(&mut self, db: &mut DataBase) {
        let stmt = db
            .conn
            .prep("SELECT location1, location2 FROM location WHERE x = :x AND y = :y")
            .expect("Error!");
        let mut result: Vec<Region>;
        loop {
            result = db
                .conn
                .exec_map(
                    &stmt,
                    params! {
                        "x" => &self.location.x,
                        "y" => &self.location.y
                    },
                    |(location1, location2)| Region {
                        location1,
                        location2,
                    },
                )
                .expect("DB Error!");

            if result.is_empty() {
                self.location.y += 1.0;
                continue;
            } else {
                break;
            }
        }

        self.region = String::from(&result[0].location2);
    }

    //기상청 API를 사용하기 위한 좌표변환
    pub fn dfs_xy_conv(v1: &f64, v2: &f64) -> LocationDfs {
        let re = RE / GRID;
        let slat1 = SLAT1 * DEGRAD;
        let slat2 = SLAT2 * DEGRAD;
        let olon = OLON * DEGRAD;
        let olat = OLAT * DEGRAD;

        let mut sn: f64 = (PI * 0.25 + slat2 * 0.5).tan() / (PI * 0.25 + slat1 * 0.5).tan();

        sn = (slat1.cos() / slat2.cos()).ln() / sn.ln();

        let mut sf = (PI * 0.25 + slat1 * 0.5).tan();

        sf = (sf.powf(sn) * slat1.cos()) / sn;

        let mut ro = (PI * 0.25 + olat * 0.5).tan();
        ro = (re * sf) / ro.powf(sn);
        let mut rs = serde_json::json!({
            "lat" : 0.0,
            "lng" : 0.0
        });

        rs["lat"] = json!(v1);
        rs["lng"] = json!(v2);
        let mut ra = (PI * 0.25 + v1 * DEGRAD * 0.5).tan();
        ra = (re * sf) / ra.powf(sn);
        let mut theta = v2 * DEGRAD - olon;

        if theta > PI {
            theta -= 2.0 * PI
        };

        if theta < -PI {
            theta += 2.0 * PI
        };

        theta *= sn;

        rs["x"] = json!((ra * theta.sin() + XO + 0.5).floor());
        rs["y"] = json!((ro - ra * theta.cos() + YO + 0.5).floor());

        let return_val: LocationDfs = serde_json::from_value(rs).expect("Error!");

        return_val
    }
}
