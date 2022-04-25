#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

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
    pub group_latitude: f64,
    pub group_longitude: f64,
    pub group_water_temp: f32,
    pub group_salinity: f32,
    pub group_height: f32,
    pub group_weight: f32,
    pub plain_buoy: i16,
}

#[derive(Serialize, Debug)]
pub struct MainGroupList {
    pub group_id: i32,
    pub group_name: String,
    pub group_latitude: f64,
    pub group_longitude: f64,
    pub group_water_temp: f32,
    pub group_salinity: f32,
    pub group_height: f32,
    pub group_weight: f32,
    pub plain_buoy: i16,
    pub smart_buoy: i16,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct WarnInfo {
    pub group_id: i16,
    pub group_name: String,
    pub line: i8,
    pub low_temp_warn: i8,
    pub high_temp_warn: i8,
    pub low_salinity_warn: i8,
    pub high_salinity_warn: i8,
    pub low_height_warn: i8,
    pub weight_warn: i8,
    pub location_warn: i8,
    pub mark: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Warn {
    pub group_id : i16,
    pub group_name : String,
    pub line : i8,
    pub warn_type : String,
    pub message : String
}