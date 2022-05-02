use serde::{Deserialize, Serialize};

//Router Params
// routes/detail_router
#[derive(Serialize)]
pub struct Obj {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Name {
    pub group: String,
}

#[derive(Deserialize, Serialize)]
pub struct BuoyListQuery {
    group: String,
}

#[derive(Deserialize, Serialize)]
pub struct BuoyQuery {
    model: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BuoyAllocate {
    model: String,
    group_name: String,
    line: i8,
}



//Router Data to Save DB
// /routes/functions/detail_data


//get_grouop_detail_data, get_group_line_data
/*
"SELECT b.group_name,
  line,
  AVG(latitude) as latitude,
  AVG(longitude) as longitude,
  AVG(water_temp) as water_temp,
  AVG(salinity) as salinity,
  AVG(height) as height,
  AVG(weight) as weight
FROM
    buoy_model a
INNER JOIN
    buoy_group b ON a.group_id = b.group_id
WHERE
    group_name = :name GROUP BY a.line"
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct GroupLineAvg {
    pub group_name: String,
    pub line: i16,
    pub latitude: f64,
    pub longitude: f64,
    pub water_temp: f64,
    pub salinity: f64,
    pub height: f64,
    pub weight: f64,
}


//get_group_history
pub struct List {
  pub group_id: i16,
  pub group_name: String,
}



//get_line_buoy_list
/*
SELECT model_idx, model, latitude, longitude, water_temp, salinity, height, weight, warn
             FROM
                 buoy_model a
             INNER JOIN
                 buoy_group b ON a.group_id = b.group_id
             WHERE
                 group_name = :group_name AND line = :line
             ORDER BY model_idx asc
*/
#[derive(Serialize)]
pub struct BuoyList {
    pub model_idx: i16,
    pub model: String,
    pub latitude: f64,
    pub longitude: f64,
    pub water_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
    pub warn: i16,
}

//get_buoy, get_buoy_list

/*
SELECT model_idx, model, line, a.group_id, b.group_name, latitude, longitude, water_temp, salinity, height, weight, warn
             FROM
                 buoy_model a
             INNER JOIN
                 buoy_group b ON a.group_id = b.group_id
             WHERE
                 group_name = :group_name
             ORDER BY model_idx asc

*/

#[derive(Serialize, Deserialize, Debug)]
pub struct BuoySpecify {
    pub model_idx: i16,
    pub model: String,
    pub line: i8,
    pub group_id: i8,
    pub group_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub water_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
    pub warn: i16,
}

/*
SELECT temp_warn, salinity_warn, height_warn, weight_warn, location_warn
            FROM
                buoy_model a
            LEFT OUTER JOIN
                buoy_group b ON a.group_id = b.group_id
            WHERE
                group_name = :group_name
            ORDER BY model_idx asc
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct BuoyWarn {
    pub temp_warn: i8,
    pub salinity_warn: i8,
    pub height_warn: i8,
    pub weight_warn: i8,
    pub location_warn: i8,
}