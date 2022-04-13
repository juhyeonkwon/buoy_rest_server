use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod db;
mod routes;
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Server run port 3124!");

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(cors)
            .service(fs::Files::new("/files", "./static"))
            .service(fs::Files::new("/swagger", "./swagger/dist/").index_file("index.html"))
            .service(routes::main_router::get_main_data)
            .service(routes::main_router::group)
            .service(routes::detail_router::group_list)
    })
    .bind(("127.0.0.1", 3124))?
    .run()
    .await
}
