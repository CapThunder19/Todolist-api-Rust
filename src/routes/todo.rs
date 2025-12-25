use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::state::AppState;
use crate::models::{Todo, NewTodo};



pub async fn get_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = state.todos.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(todos.clone()))
}

pub async fn add_todo(State(state): State<AppState>, Json(new_todo): Json<NewTodo>) -> Result<Json<Todo>, StatusCode> {
    if new_todo.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut todos = state.todos.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo = Todo {
        id: todos.len() + 1,
        title: new_todo.title,
        completed: false,   
    };

    todos.push(todo.clone());
    Ok(Json(todo))
}

pub async fn complete_todo(State(state): State<AppState>, Path(id): Path<usize>) -> Result<Json<Todo>, StatusCode> {
    let mut todos = state.todos.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match todos.iter_mut().find(|todo| todo.id == id){
        Some(todo) => {
            todo.completed = true;
            Ok(Json(todo.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
pub async fn get_todo_by_id(State(state): State<AppState>, Path(id): Path<usize>) -> Result<Json<Todo>, StatusCode> {
    let todos = state.todos.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match todos.iter().find(|todo| todo.id == id) {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }

}

pub async fn delete_todo(State(state): State<AppState>, Path(id): Path<usize>) -> Result<StatusCode, StatusCode> {
    let mut todos = state.todos.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let index = todos.iter().position(|todo| todo.id == id);
    match index {
        Some(i) => {
            todos.remove(i);
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}