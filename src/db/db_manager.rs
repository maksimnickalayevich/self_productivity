use sqlx::{Pool, Postgres};
use crate::{models::users::Users, db::query_getter::QueryGetter};

pub struct DbManager {
    pub db: Pool<Postgres>
}
impl DbManager {
    pub async fn init_tables(pool: &Pool<Postgres>) {
        let all_tables: [DbTables; 1] = [
            DbTables::Users(Users::get_table())
        ];
        for table_name in &all_tables {
            match table_name {
                DbTables::Users(users_name) => {
                    let q: String = Users::get_table_init_query( users_name);
                    sqlx::query(&q).execute(pool).await.expect("Unable to create users table")
                },
            };

        println!("Tables for DB were initialized");
        }
    }
}

#[derive(Clone)]
pub enum DbTables {
    Users(String)
}