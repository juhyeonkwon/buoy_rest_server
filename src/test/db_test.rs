#[cfg(test)]
mod tests {
    use mysql::prelude::*;
    use mysql::*;
    use serde::{Deserialize, Serialize};

    use crate::db::maria_lib::DataBase;
    use crate::db::model::{Buoy, Group};
    use dotenv::dotenv;

    #[test]
    fn group_router_test() {
        dotenv().ok();
        let mut db = DataBase::init();

        let query = r"SELECT * FROM buoy_group";

        let row: Vec<Group> = db
            .conn
            .query_map(
                query,
                |(
                    group_id,
                    group_name,
                    group_latitude,
                    group_longitude,
                    group_water_temp,
                    group_salinity,
                    group_height,
                    group_weight,
                )| Group {
                    group_id,
                    group_name,
                    group_latitude,
                    group_longitude,
                    group_water_temp,
                    group_salinity,
                    group_height,
                    group_weight,
                },
            )
            .expect("select Error");

        println!("{:#?}", row);
    }

    use crate::db::redis_lib::connect_redis;
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
}
