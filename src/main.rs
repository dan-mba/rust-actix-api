use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::Serialize;
use log::info;
use std::env;


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
    let ip_address = match env::var("IP_ADDRESS") {
        Ok(p) => p,
        Err(_e) => String::from("127.0.0.1"),
    };
    
    let port_str = match env::var("PORT") {
        Ok(p) => p,
        Err(_e) => String::from("8080"),
    };
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_e) => 8080 as u16,
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting HTTP server at http://{ip_address}:{port}");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/ping").route(web::get().to(ping)))
    })
    .workers(1)
    .bind((ip_address, port))?
    .run()
    .await
}
