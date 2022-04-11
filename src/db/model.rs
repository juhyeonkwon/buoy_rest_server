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