use actix_web::web::Data;
use actix_web::{HttpResponse, Result};

use super::Todo;
use crate::db_pool::{DBError, DBPool};
use crate::page::Page;

pub async fn list(dbpool: Data<DBPool>) -> Result<HttpResponse> {
    let mut db = dbpool.get_connection().await?;
    let todos = sqlx::query_as!(
        Todo,
        r#"select todo_id, name, state as "state: _" from todos"#
    )
    .fetch_all(&mut db)
    .await
    .map_err(|err| {
        println!("{err}");
        DBError::Read
    })?;

    Ok(HttpResponse::Ok().json(Page {
        size: todos.len(),
        items: todos,
    }))
}
