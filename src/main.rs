use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

extern crate env_logger;

mod db;
mod routes;
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Server run port 3124!");

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(fs::Files::new("/files", "./static"))
            .service(fs::Files::new("/swagger", "./swagger/dist/").index_file("index.html"))
            .service(routes::main_router::get_location_data)
            .service(routes::main_router::get_main_data_region)
            .service(routes::main_router::get_sky_data)
            .service(routes::main_router::group)
            .service(routes::main_router::group_total)
            .service(routes::main_router::get_location_data)
            .service(routes::detail_router::group_list)
            .service(routes::detail_router::group_detail)
            .service(routes::detail_router::buoy_detail)
    })
    .bind(("192.168.0.20", 3124))?
    .run()
    .await
}
