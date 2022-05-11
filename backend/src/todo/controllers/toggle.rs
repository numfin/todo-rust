use actix_web::web::{self, Data};
use actix_web::{HttpResponse, Result};
use ormx::Table;
use sqlx::query_as;
use uuid::Uuid;

use crate::db_pool::{DBError, DBPool};
use crate::todo::models::{TodoState, UpdateTodo};

pub async fn toggle(dbpool: Data<DBPool>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let mut db = dbpool.get_connection().await?;
    let todo_id = path.into_inner();

    let mut todo = query_as!(
        super::Todo,
        r#"select todo_id, name, state as "state: _" from todos where todo_id=$1"#,
        todo_id
    )
    .fetch_one(&mut db)
    .await
    .map_err(|err| {
        println!("{err}");
        DBError::Read
    })?;

    todo.patch(
        &mut db,
        UpdateTodo {
            state: match todo.state {
                TodoState::Active => TodoState::Done,
                TodoState::Done => TodoState::Active,
            },
        },
    )
    .await
    .map_err(|err| {
        println!("{err}");
        DBError::Read
    })?;
    Ok(HttpResponse::Ok().json(todo))
}
