use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    dev::Service as _, middleware, post, web, App, HttpMessage, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use futures_util::future::FutureExt;

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
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(
                web::scope("/main")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::main_router::get_location_data)
                    .service(routes::main_router::get_main_data_region)
                    .service(routes::main_router::get_sky_data)
                    .service(routes::main_router::group)
                    .service(routes::main_router::group_total)
                    .service(routes::main_router::get_location_data)
                    .service(routes::main_router::get_main_warn),
            )
            .service(
                web::scope("/detail")
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::detail_router::group_list)
                    .service(routes::detail_router::group_detail)
                    .service(routes::detail_router::group_history)
                    .service(routes::detail_router::buoy_spec)
                    .service(routes::detail_router::buoy_detail)
                    .service(routes::detail_router::buoy_group_list)
                    .service(routes::detail_router::buoy_allocate)
                    .service(routes::detail_router::buoy_deallocate),
            )
            .service(
                web::scope("/user")
                    .service(routes::auth_router::login)
                    .service(routes::auth_router::register)
                    .service(routes::auth_router::check_duple)
                    .service(routes::auth_router::send_key)
                    .service(routes::auth_router::email_auth),
            )
            .service(fs::Files::new("/files", "./static"))
            .service(fs::Files::new("/swagger", "./swagger/dist/").index_file("index.html"))
            .service(routes::etc_router::get_main_warn)
            .service(
                web::scope("/etc")
                    // .wrap_fn(|req, srv| {
                    //     let value = req.headers().get("Authorization");
                    //     let token : Option<String> = match value {
                    //         Some(v) => {
                    //             let _temp = v.to_str().unwrap_or_default();
                    //             let _split : Vec<&str> = _temp.split("Bearer").collect();
                    //             Some(String::from(_split[1].trim()))
                    //         },
                    //         None => None
                    //     };
                    //     req.extensions_mut().insert(token);
                    //     srv.call(req).map(|res| {
                    //         println!("Hi from response");
                    //         res
                    //     })
                    // })
                    .wrap(custom_middleware::jwt::GetUserValue)
                    .service(routes::etc_router::get_test),
            )
    })
    .bind(("192.168.0.20", 3124))?
    .run()
    .await
}
