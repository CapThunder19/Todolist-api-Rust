
use std::sync::{Mutex, Arc};
use crate::models::{Todo, NewTodo};

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}