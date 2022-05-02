use crate::db::redis_lib::connect_redis;
use redis::Commands;

use crate::db::model::common_model::Warn;

pub fn get_warn_alarm_list() -> Vec<Warn> {
    let mut conn = connect_redis();

    let warn_text: String = match redis::cmd("GET").arg("warn_alarm_list").query(&mut conn) {
        Ok(v) => v,
        Err(_) => String::from("{}"),
    };

    match serde_json::from_str(&warn_text) {
        Ok(v) => v,
        Err(_) => Vec::new(),
    }
}
