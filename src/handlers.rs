use actix_web::{web, Responder};
use deadpool_postgres::{Pool, Client};

use super::{models, db};

mod json;

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(models::Status::new("UP"))
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error Connecting to the database");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => web::HttpResponse::Ok().json(todos),
        Err(_) => web::HttpResponse::InternalServerError().into(),
    }
}


pub async fn get_todo_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error Connecting to the database");

    let result = db::get_todo_items(&client, path.0).await;

    match result {
        Ok(items) => web::HttpResponse::Ok().json(items),
        Err(_) => web::HttpResponse::InternalServerError().into(),
    }
}


pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<json::create::TodoList>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error Connecting to the database");

    let result = db::create_todo(&client, json.title.clone()).await;

    match result {
        Ok(todo) => web::HttpResponse::Ok().json(todo),
        Err(_) => web::HttpResponse::InternalServerError().into(),
    }
}
