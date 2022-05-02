#[cfg(test)]
mod tests {
    use mysql::prelude::*;
    use mysql::*;

    use crate::db::maria_lib::DataBase;
    use crate::db::model::*;
    use crate::db::redis_lib::connect_redis;
    use dotenv::dotenv;
    use redis::Commands;

    #[test]
    fn redis_test() {
        dotenv().ok();
        let mut conn = connect_redis();

        let mut a: String = String::from("");
        let _: () = match conn.get("main_data_set") {
            Ok(v) => a = v,
            Err(_) => println!("Error!"),
        };

        println!("{}", a);
    }

    use crate::routes::functions::main_data::Distance;

    #[test]
    fn maria_query_map_test() {
        dotenv().ok();

        let mut db = DataBase::init();

        let stmt = db.conn.prep("SELECT
        (6371 * acos(cos(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0) * cos(:lat * 3.141592653589793 / 180.0)
        * cos((:lon * 3.141592653589793 / 180.0) - (CAST(lon AS FLOAT) * 3.141592653589793 / 180.0)) + sin(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0)
        * sin(:lat * 3.141592653589793 / 180.0))) as distance, number, name
        FROM observation_list where tide_level = 1 AND w_temperature = 1 AND salinity = 1 AND air_temperature = 1 AND wind_velocity = 1 order BY distance asc").expect("Db prep Error!");

        let data: Vec<Distance> = db
            .conn
            .exec_map(
                stmt,
                params! {
                  "lat" => 35.1513466,
                  "lon" => 128.1001125,
                },
                |(distance, number, name)| Distance {
                    distance,
                    number,
                    name,
                },
            )
            .expect("Error!");

        let mut conn = connect_redis();

        let _key = String::from("obs_") + &data[0].number;
        let mut a: String = String::from("");
        let _: () = match conn.get(_key) {
            Ok(v) => a = v,
            Err(_) => println!("Error!"),
        };

        let obs_data: serde_json::Value = serde_json::from_str(&a).expect("Error!");
        println!("{:#?}", obs_data);

        let stmt = db.conn.prep("SELECT
        (6371 * acos(cos(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0) * cos(:lat * 3.141592653589793 / 180.0)
        * cos((:lon * 3.141592653589793 / 180.0) - (CAST(lon AS FLOAT) * 3.141592653589793 / 180.0)) + sin(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0)
        * sin(:lat * 3.141592653589793 / 180.0))) as distance, number, name
        FROM observation_list where digging = 1 order BY distance asc").expect("Db prep Error!");

        let wave: Vec<Distance> = db
            .conn
            .exec_map(
                stmt,
                params! {
                  "lat" => 35.1513466,
                  "lon" => 128.1001125,
                },
                |(distance, number, name)| Distance {
                    distance,
                    number,
                    name,
                },
            )
            .expect("Error!");

        let _key = String::from("wave_hight_") + &wave[0].number;

        let _: () = match conn.get(_key) {
            Ok(v) => a = v,
            Err(_) => println!("Error!"),
        };

        let wave_data: serde_json::Value = serde_json::from_str(&a).expect("Error!");
        println!("{:#?}", wave_data);
    }

    use crate::db::model::common_model::TideBuoy;
    use crate::db::model::common_model::TideRader;
    use crate::db::model::common_model::TideRaderList;

    use serde_json::json;
    #[test]
    fn tide_velocity_test() {
        dotenv().ok();

        let mut db = DataBase::init();

        let stmt = db.conn.prep("SELECT
        (6371 * acos(cos(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0) * cos(:lat * 3.141592653589793 / 180.0)
        * cos((:lon * 3.141592653589793 / 180.0) - (CAST(lon AS FLOAT) * 3.141592653589793 / 180.0)) + sin(CAST(lat AS FLOAT) * 3.141592653589793 / 180.0)
        * sin(:lat * 3.141592653589793 / 180.0))) as distance, number, name
        FROM observation_list where tide_velocity > 0 order BY distance asc").expect("db prep Error!");

        let data: Vec<Distance> = db
            .conn
            .exec_map(
                stmt,
                params! {
                  "lat" => 35.1513466,
                  "lon" => 128.1001125,
                },
                |(distance, number, name)| Distance {
                    distance,
                    number,
                    name,
                },
            )
            .expect("Error!");

        let mut conn = connect_redis();

        let mut a: String = String::from("");

        //HF인지 아닌지 체크
        let mut tide_type: String = String::from("");

        for val in data {
            let _key = String::from("tidal_") + &val.number;
            tide_type = String::from(&val.number[0..2]);
            let _: () = match conn.get(_key) {
                Ok(v) => a = v,
                Err(_) => {
                    println!("not founed in redis");
                    continue;
                }
            };
            break;
        }

        if tide_type == String::from("HF") {
            let value: TideRaderList = serde_json::from_str(&a).expect("Error!");
            println!("{:#?}, {}", value, tide_type);
        } else {
            let value: TideBuoy = serde_json::from_str(&a).expect("Error!");
            let return_value: serde_json::Value = json!({
                "current_direct" : value.current_direct.parse::<f64>().expect("err"),
                "current_speed" : value.current_speed.parse::<f64>().expect("err"),
            });
            println!("{:#?}, {}", return_value, tide_type);
        }
        // let obs_data : serde_json::Value = serde_json::from_str(&a).expect("Error!");
    }

    #[test]
    fn get_neareast_hf_test() {
        dotenv().ok();
        let mut conn = connect_redis();

        let mut a: String = String::from("");

        let _key = String::from("tidal_HF_0063");
        let _: () = match conn.get(_key) {
            Ok(v) => a = v,
            Err(_) => {
                println!("Error!");
            }
        };

        let list: Vec<TideRader> = serde_json::from_str(&a).expect("error");
        let mut min: f64 = 300.0;
        let mut current = 0;
        for (i, val) in list.iter().enumerate() {
            let dis: f64 = get_distance(
                (35.1513466, 128.1001125),
                (val.lat.parse().expect("Err"), val.lon.parse().expect("Err")),
            );

            if dis < min {
                min = dis;
                current = i;
            }
        }

        println!("{:#?}", list[current]);
    }

    fn get_distance(center: (f64, f64), target: (f64, f64)) -> f64 {
        let earth_radius_kilometer = 6371.0_f64;
        let (center_latitude_degrees, center_longitude_degrees) = center;
        let (target_latitude_degrees, target_longitude_degrees) = target;

        let center_latitude = center_latitude_degrees.to_radians();
        let target_latitude = target_latitude_degrees.to_radians();

        let delta_latitude = (center_latitude_degrees - target_latitude_degrees).to_radians();
        let delta_longitude = (center_longitude_degrees - target_longitude_degrees).to_radians();

        let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
            + center_latitude.cos() * target_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
        let central_angle = 2.0 * central_angle_inner.sqrt().asin();

        let distance = earth_radius_kilometer * central_angle;

        distance
    }

    use crate::routes::functions::main_data::get_near_obs_data;
    use crate::routes::functions::main_data::get_near_tide_data;
    use crate::routes::functions::main_data::get_near_wave_data;

    #[test]
    fn get_near_data_test() {
        dotenv().ok();
        let mut db = DataBase::init();
        let mut conn = connect_redis();

        let obs_val: serde_json::Value =
            get_near_obs_data(&mut db, &mut conn, &35.1513466, &128.1001125);
        let wave_val: serde_json::Value =
            get_near_wave_data(&mut db, &mut conn, &35.1513466, &128.1001125);
        let tide_val: serde_json::Value =
            get_near_tide_data(&mut db, &mut conn, &35.1513466, &128.1001125);

        println!("{:#?}", obs_val);
        println!("{:#?}", wave_val);
        println!("{:#?}", tide_val);
    }

    use crate::routes::functions::detail_data::get_group_line_data;
    #[test]
    fn get_group_test() {
        dotenv().ok();
        let mut db = DataBase::init();
        get_group_line_data(&mut db, &String::from("A"));
    }

    use chrono;
    use chrono::prelude::*;
    use chrono::Duration;

    #[derive(Debug)]
    struct Line {
        model: String,
        line: i16,
    }
    #[test]
    fn redis_get_line_avg_history() {
        dotenv().ok();

        let mut db = DataBase::init();

        let query =
            r"SELECT model, line FROM buoy_model WHERE group_id = 1 AND line = 1 order by line";

        let row: Vec<Line> = db
            .conn
            .query_map(query, |(model, line)| Line { model, line })
            .expect("select Error");

        let mut conn = connect_redis();

        for val in row.iter() {
            let a: Vec<String> = redis::cmd("LRANGE")
                .arg(&val.model)
                .arg("0")
                .arg("6")
                .query(&mut conn)
                .expect("Error!");
            println!("{:#?}", a);
        }

        // let a : Vec<String> = redis::cmd("LRANGE").arg("buoy_1").arg("0").arg("6").query(&mut conn).expect("Error!");

        // println!("{:#?}", row);
    }

    #[derive(Debug)]
    struct WarnInfo {
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
    #[test]
    fn warn_test() {
        dotenv().ok();

        let mut db = DataBase::init();

        let group_list : Vec<WarnInfo> = db.conn.query_map("SELECT a.group_id, b.group_name,
                                                                    a.line, 
                                                                    SUM(temp_warn = 1) AS low_temp_warn,  
                                                                    SUM(temp_warn = 2) AS high_temp_warn,
                                                                    SUM(salinity_warn = 1) AS low_salinity_warn,
                                                                    SUM(salinity_warn = 2) AS high_salinity_warn,
                                                                    SUM(height_warn = 1) AS low_height_warn,
                                                                    SUM(weight_warn = 1) AS weight_warn,
                                                                    SUM(location_warn = 1) AS location_warn,
                                                                    COUNT(*) * 0.5 AS mark 
                                                            FROM buoy_model a, buoy_group b WHERE a.group_id = b.group_id AND a.group_id = 1 AND a.group_id > 0 GROUP BY line", |(
                                                                group_id ,
                                                                group_name,
                                                                line,
                                                                low_temp_warn,
                                                                high_temp_warn,
                                                                low_salinity_warn,
                                                                high_salinity_warn,
                                                                low_height_warn,
                                                                weight_warn,
                                                                location_warn,
                                                                mark
                                                            )| WarnInfo {
                                                                group_id,
                                                                group_name,
                                                                line,
                                                                low_temp_warn,
                                                                high_temp_warn,
                                                                low_salinity_warn,
                                                                high_salinity_warn,
                                                                low_height_warn,
                                                                weight_warn,
                                                                location_warn,
                                                                mark
        }).expect("Error!");

        println!("{:#?}", group_list);
    }
}
