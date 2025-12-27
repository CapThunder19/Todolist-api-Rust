
use std::sync::{Mutex, Arc};
use crate::models::{Todo, NewTodo};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}