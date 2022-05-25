use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    dev::Service as _, middleware, post, web, App, HttpMessage, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use futures_util::future::FutureExt;

use std::sync::Arc;

extern crate env_logger;

mod custom_middleware;
mod db;
mod routes;
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Server run port 3124!");

    HttpServer::new(|| {
        let pool = db::maria_lib::DataBase::init().pool;
        let redis_conn = db::redis_lib::get_client();

        // let temp = web::Data::new(Mutex::new(db.pool));

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
        
            .wrap(middleware::Logger::default())
            .wrap(cors)
            // .app_data(web::Data::clone(&temp))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_conn.clone()))
            .service(
                web::scope("/main")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::main_router::get_location_data)    //get, main/data?
                    .service(routes::main_router::group)                //get, main/group
                    .service(routes::main_router::group_total)          //get, main/group/total
                    .service(routes::main_router::get_main_warn)        //get, main/warn
                    //legacy
                    .service(routes::main_router::get_main_data_region)
                    .service(routes::main_router::get_sky_data),
            )
            .service(
                web::scope("/detail")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::group_list)         //get, detail/group/list
                    .service(routes::detail_router::group_detail)       //get, detail/group?group_id
                    .service(routes::detail_router::group_detail_web)   //get, detail/group/web?group_id
                    .service(routes::detail_router::group_history)      //get, detail/group/history?group_id
                    .service(routes::detail_router::group_modify)       //put, detail/group/modify
                    .service(routes::detail_router::create_group)       //post, detail/group/create
                    .service(routes::detail_router::delete_group)       //delete detail/group/delete
                    .service(routes::detail_router::buoy_spec)          //get, detail/buoy?
                    .service(routes::detail_router::buoy_group_list)    //get, detail/buoy/list
                    .service(routes::detail_router::buoy_detail)        //get, detail/buoy/history
                    .service(routes::detail_router::buoy_assigned)      //get, detail/buoy/assigned
                    .service(routes::detail_router::buoy_unassigned)    //get, detail/buoy/unassigned
                    .service(routes::detail_router::buoy_allocate)      //put, detail/buoy/allocate
                    .service(routes::detail_router::buoy_allocate_list)  //put, detail/buoy/allocate/list
                    .service(routes::detail_router::buoy_deallocate)   //put, detail/buoy/deallocate
                    .service(routes::detail_router::buoy_deallocate_list),   //put, detail/buoy/deallocate/list
            )
            .service(
                web::scope("/user")
                    .service(routes::auth_router::login)                //post, user/login
                    .service(routes::auth_router::register)             //post, user/register
                    .service(routes::auth_router::check_duple)          //post, user/check 
                    .service(routes::auth_router::send_key)             //post, user/email/key
                    .service(routes::auth_router::email_auth)         //post, user/email/auth    
                    .service(routes::auth_router::oauth_register)         //post, user/oauth/
                    .service(routes::auth_router::google)          //post, user/oauth/
                    .service(routes::auth_router::kakao)          //post, user/oauth/
                    .service(routes::auth_router::naver)          //post, user/oauth/
            )
            .service(
                web::scope("/manage")
                    .wrap(custom_middleware::jwt::GetUserValue)         
                    .service(routes::manage_router::user_list)          //get, manage/user/list
                    .service(routes::manage_router::user_modify)        //put, manage/user/modify
                    .service(routes::manage_router::user_delete)        //delete, manage/user/delete
                    .service(routes::manage_router::buoy_unassigned)    //get, manage/buoy/unassigned
                    .service(routes::manage_router::buoy_allocate)      //put, manage/buoy/allocate
                    .service(routes::manage_router::buoy_deallocate)    //put, manage/buoy/deallocate
            )
            .service(
                web::scope("/setting")
                    .wrap(custom_middleware::jwt::GetUserValue)         
                    .service(routes::user_setting_router::modify)       //put, setting/user/password
            )
            .service(fs::Files::new("/files", "./static"))
            .service(fs::Files::new("/swagger", "./swagger/dist/").index_file("index.html"))
            .service(routes::etc_router::get_main_warn)
            .service(
                web::scope("/etc")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::etc_router::get_test),
            )
    })
    .workers(4)
    .bind(("192.168.0.20", 3124))?
    .bind(("localhost", 3124))?
    .run()
    .await
}
