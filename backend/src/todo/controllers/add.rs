use super::InsertTodo;
use actix_web::web::{self, Data};
use actix_web::{HttpResponse, Result};
use ormx::Insert;
use uuid::Uuid;

use crate::db_pool::{DBError, DBPool};
use crate::todo::models::{TodoInput, TodoState};

pub async fn add(dbpool: Data<DBPool>, body: web::Json<TodoInput>) -> Result<HttpResponse> {
    let mut db = dbpool.get_connection().await?;

    let new_todo = InsertTodo {
        todo_id: Uuid::new_v4(),
        name: body.name.clone(),
        state: TodoState::Active,
    }
    .insert(&mut db)
    .await
    .map_err(|err| {
        println!("{err}");
        DBError::Write
    })?;
    Ok(HttpResponse::Ok().json(new_todo))
}
