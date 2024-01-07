use actix_web::{
    get,
    post, 
    HttpResponse,
    web::{Json, Data, Path},
};
use sqlx;

use crate::{models::users::Users, db::db_manager::DbManager};

#[post("/register")]
pub async fn register_user(db: Data<DbManager>, user: Json<Users>) -> HttpResponse {
    let insert_query: String = Users::insert_one_query();
    // TODO: Check authorization/authentication here via headers
    // TODO: Add validations here
    // TODO: Handle duplicates
    let result = sqlx::query_as::<_, Users>(
    &insert_query    
    )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.raw_password)
        .bind(&user.email)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .fetch_one(&db.db)
        .await;
    match result {
        Ok(created_user) => HttpResponse::Created().json(created_user),
        Err(e) => {
            println!("Failed to insert user: {e}");
            HttpResponse::BadRequest().json("Unable to insert user")  // TODO: Create custom error object
        } 
    }
}