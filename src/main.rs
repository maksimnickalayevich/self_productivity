use crate::configs::server_config::ServerConfig;
use db::{repo::Repository, db_manager::DbManager};
use api::users::register_user;
use actix_web::{
    App, 
    HttpServer,
    web::{self, Data}
};
use sqlx::{Pool, Postgres};

mod api;
mod configs;
mod db;
mod middlewares;
mod models;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configs from environment
    dotenvy::dotenv().ok();
    let server_config: ServerConfig = ServerConfig::load_config();
    let mut repo: Repository = Repository::new();
    let conn_pool: Pool<Postgres> = repo.start_connection_pool().await;
    DbManager::init_tables(&conn_pool).await;
    
    // Init actix application
    let server = HttpServer::new(move || {
        let users_scope = web::scope("/api/users")
            .service(register_user);

        App::new()
        .app_data(Data::new(DbManager{db: conn_pool.clone()}))
        .service(users_scope)
    })
    .bind(
        (server_config.server_address.clone(), server_config.server_port)
    )?
    .run();

    println!();
    println!("Server is running on http://{}:{}", server_config.server_address, server_config.server_port);
    server.await
}
