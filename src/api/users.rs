use actix_web::{get, HttpResponse};

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello from actix web!")
}