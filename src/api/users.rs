use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello from actix web!")
}