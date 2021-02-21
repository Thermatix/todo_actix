use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use std::io;

use super::models::*;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoLists>, io::Error> {

    let statement = client.prepare("SELECT * FROM todo_lists order by id asc").await.unwrap();

    let todos = client.query(&statement, &[])
                      .await
                      .expect("Error getting todo lists")
                      .iter()
                      .map(|row| TodoLists::from_row_ref(row).unwrap())
                      .collect::<Vec<TodoLists>>();
    Ok(todos)
}

pub async fn get_todo_items(client: &Client, list_id: i32) -> Result<Vec<TodoItems>, io::Error> {
    let statement = client.prepare("SELECT * FROM todo_items where list_id = $1 order by id asc").await.unwrap();

    let items = client.query(&statement, &[&list_id])
                      .await
                      .expect("Error getting todo lists")
                      .iter()
                      .map(|row| TodoItems::from_row_ref(row).unwrap())
                      .collect::<Vec<TodoItems>>();
    Ok(items)
}
