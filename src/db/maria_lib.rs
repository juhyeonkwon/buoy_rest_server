use mysql::*;
use std::env;

// use actix_web::web::Json;

pub struct DataBase {
    pub pool: Pool,
    pub conn: PooledConn,
}

impl DataBase {
    pub fn init() -> DataBase {
        let user = env::var("MYSQL_USER_NAME").expect("ENV not Found");
        let password = env::var("MYSQL_PASSWORD").expect("ENV not Found");
        let ip = env::var("MYSQL_IP").expect("ENV not Found");
        let port = env::var("MYSQL_PORT").expect("").parse().unwrap();
        let db_name = env::var("MYSQL_DB_NAME").expect("ENV not Found");

        let opts = OptsBuilder::new()
            .user(Some(user))
            .pass(Some(password))
            .ip_or_hostname(Some(ip))
            .tcp_port(port)
            .db_name(Some(db_name));

        let pool = Pool::new(opts).unwrap();
        let conn = pool.get_conn().unwrap();

        DataBase { pool, conn }
    }
}
