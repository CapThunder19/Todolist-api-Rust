use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    state::AppState,
    models::{Todo, NewTodo},
};

use sqlx::Row;

pub async fn get_todos( State(state): State<AppState>,) -> Result<Json<Vec<Todo>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, title, completed FROM todos"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todos = rows
        .into_iter()
        .map(|row| Todo {
            id: row.get("id"),
            title: row.get("title"),
            completed: row.get("completed"),
        })
        .collect();

    Ok(Json(todos))
}

pub async fn add_todo( State(state): State<AppState>, Json(new_todo): Json<NewTodo>, ) -> Result<Json<Todo>, StatusCode> {
    if new_todo.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = sqlx::query(
        "INSERT INTO todos (title, completed) VALUES (?, false)"
    )
    .bind(&new_todo.title)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.last_insert_rowid();

    Ok(Json(Todo {
        id: id as usize,
        title: new_todo.title,
        completed: false,
    }))
}

pub async fn get_todo_by_id( State(state): State<AppState>, Path(id): Path<usize>, ) -> Result<Json<Todo>, StatusCode> {
    let row = sqlx::query(
        "SELECT id, title, completed FROM todos WHERE id = ?"
    )
    .bind(id as i64)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => Ok(Json(Todo {
            id: row.get("id"),
            title: row.get("title"),
            completed: row.get("completed"),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn complete_todo( State(state): State<AppState>, Path(id): Path<usize>, ) -> Result<Json<Todo>, StatusCode> {
    let result = sqlx::query(
        "UPDATE todos SET completed = true WHERE id = ?"
    )
    .bind(id as i64)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let row = sqlx::query(
        "SELECT id, title, completed FROM todos WHERE id = ?"
    )
    .bind(id as i64)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Todo {
        id: row.get("id"),
        title: row.get("title"),
        completed: row.get("completed"),
    }))
}

pub async fn delete_todo( State(state): State<AppState>,Path(id): Path<usize>, ) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM todos WHERE id = ?"
    )
    .bind(id as i64)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
