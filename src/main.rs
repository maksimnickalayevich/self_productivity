use actix_web::{HttpServer, App};
use api::users::{hello};

mod api;
mod db;
mod models;
mod utilities;
mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 5050))?
    .run()
    .await
}
