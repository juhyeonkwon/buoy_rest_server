use serde::{Deserialize, Serialize};

//Router Params
// routes/main_router
#[derive(Serialize, Deserialize)]
pub struct RealLocation {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub location: String,
}

#[derive(Serialize, Deserialize)]
pub struct Total {
    pub water_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
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
