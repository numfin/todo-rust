use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, ormx::Table)]
#[ormx(table = "todos", id = todo_id, insertable)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Todo {
    pub todo_id: Uuid,
    pub name: String,
    #[ormx(custom_type)]
    pub state: TodoState,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "todo_state")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoState {
    Active,
    Done,
}

#[derive(Debug, Deserialize)]
pub struct TodoInput {
    pub name: String,
}

#[derive(ormx::Patch)]
#[ormx(table = Todo, table_name = "todos", id = "todo_id")]
pub struct UpdateTodo {
    #[ormx(custom_type)]
    pub state: TodoState,
}
