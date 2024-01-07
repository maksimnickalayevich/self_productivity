use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::db::query_getter::QueryGetter;

#[derive(Serialize, Deserialize, Debug, Default, FromRow)]
pub struct Users {
    pub id: i64,
    pub username: String,
    pub raw_password: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    // TODO: Add refs to other tables (Tasks, Posts, Comments) between tables
}

impl Users {
    pub fn insert_one_query() -> String {
        let query = format!(r#"
        INSERT INTO {} (id, username, raw_password, email, first_name, last_name)
        VALUES ($1, $2, $3, $4, $5, $6) RETURNING 
        id, 
        username, 
        raw_password,
        email,
        first_name,
        last_name 
        "#, Users::get_table());
        return query;
    }
    pub fn get_table() -> String {
        String::from("users")
    }
}

impl <'a> QueryGetter<'a, Users> for Users {
    fn get_table_init_query(table_name: &'a String) -> String {
        let query: String = format!(r#"
        CREATE TABLE IF NOT EXISTS {} (
          id bigserial PRIMARY KEY NOT NULL,
          username VARCHAR(64) NOT NULL,
          raw_password VARCHAR(64) NOT NULL,
          email VARCHAR(32),
          first_name VARCHAR(32),
          last_name VARCHAR(32)
        );"#, table_name);

        println!("Query for initializing Users table were created! {}", query);
        return query;
    }
}