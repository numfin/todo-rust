use actix_web::web::{self, Data};
use actix_web::{HttpResponse, Result};
use sqlx::query;
use uuid::Uuid;

use crate::db_pool::{DBError, DBPool};

pub async fn delete(dbpool: Data<DBPool>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let mut db = dbpool.get_connection().await?;
    let todo_id = path.into_inner();

    query!("delete from todos where todo_id=$1", todo_id)
        .execute(&mut db)
        .await
        .map_err(|err| {
            println!("{err}");
            DBError::Delete
        })?;
    Ok(HttpResponse::Ok().body("Done"))
}
