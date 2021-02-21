use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use std::io;

use super::models::*;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoLists>, io::Error> {

    let statement = client.prepare("SELECT * FROM todo_lists ORDER BY id ASC").await.unwrap();

    let todos = client.query(&statement, &[])
                      .await
                      .expect("Error getting todo lists")
                      .iter()
                      .map(|row| TodoLists::from_row_ref(row).unwrap())
                      .collect::<Vec<TodoLists>>();
    Ok(todos)
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoLists, io::Error> {
    let statement = client.prepare("INSERT INTO todo_lists (title) VALUES ($1) RETURNING id, title").await.unwrap();

    client.query(&statement, &[&title])
          .await
          .expect("Error creating todo list")
          .iter()
          .map(|row| TodoLists::from_row_ref(row).unwrap())
          .collect::<Vec<TodoLists>>()
          .pop()
          .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating todo list"))

}

pub async fn get_todo_items(client: &Client, list_id: i32) -> Result<Vec<TodoItems>, io::Error> {
    let statement = client.prepare("SELECT * FROM todo_items WHERE list_id = $1 ORDER BY id ASC").await.unwrap();

    let items = client.query(&statement, &[&list_id])
                      .await
                      .expect("Error getting todo lists")
                      .iter()
                      .map(|row| TodoItems::from_row_ref(row).unwrap())
                      .collect::<Vec<TodoItems>>();
    Ok(items)
}



pub async fn check_todo_item(client: &Client, list_id: i32, item_id: i32)
    -> Result<(), io::Error> {

    let statement = client.prepare("UPDATE todo_items SET checked = NOT checked WHERE list_id = $1 AND id = $2").await.unwrap();

    let result = client.execute(&statement, &[&list_id, &item_id])
                        .await
                        .expect("Error checking todo item");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check the item"))
    }
}
