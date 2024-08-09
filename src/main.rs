#![allow(dead_code)]
mod gen_errors;
mod player;
mod templates;
mod index;
mod websocket;
mod game;
use actix_files as afs;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use index::index_controller;


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                afs::Files::new("/public", "./assets/public").show_files_listing()
            )
            .service(hello)
            .service(player::player_controller::get_route_config())
            .service(index_controller::index_controller)
    })
    .workers(1)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
