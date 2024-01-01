use tokio_pg_mapper_derive::PostgresMapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PostgresMapper, Debug, Default)]
#[pg_mapper(table="users")]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub raw_password: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    // TODO: Add refs to other tables (Tasks, Posts, Comments) between tables
}
