use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "todo_lists")]
pub struct TodoLists {
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "todo_items")]
pub struct TodoItems {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}

impl Status {
    pub fn new(given_status: &str) -> Self {
        Self {
            status: given_status.to_string(),
        }
    }
}
