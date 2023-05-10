use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::Serialize;
use log::info;

#[derive(Serialize)]
struct MyResp {
    message: String,
}

async fn ping() -> HttpResponse {
    let resp = MyResp {
        message: String::from("pong")
    };
    HttpResponse::Ok().json(resp) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/ping").route(web::get().to(ping)))
    })
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
