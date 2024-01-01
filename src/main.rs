use crate::configs::server_config::ServerConfig;
use db::repo::Repository;
use api::users::hello;
use actix_web::{
    App, 
    HttpServer
};
use sqlx::{Pool, Postgres};

mod api;
mod configs;
mod db;
mod middlewares;
mod models;

// TODO: Init database with tables

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configs from environment
    dotenvy::dotenv().ok();
    let server_config: ServerConfig = ServerConfig::load_config();
    let mut repo: Repository = Repository::new();
    let _: Pool<Postgres> = repo.start_connection_pool().await;
    
    // Init actix application
    let server = HttpServer::new(move || {
        App::new()
        .service(hello)
    })
    .bind(
        (server_config.server_address.clone(), server_config.server_port)
    )?
    .run();

    println!();
    println!("Server is running on http://{}:{}", server_config.server_address, server_config.server_port);
    server.await
}
