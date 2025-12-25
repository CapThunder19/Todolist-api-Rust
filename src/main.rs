use axum::{
    routing::{get,post,patch, delete},
    Router,
};


use std::sync::{Mutex, Arc};

mod models;
mod routes;
mod state;

use state::AppState;
use routes::todo::{get_todos, add_todo, complete_todo, get_todo_by_id, delete_todo};



#[tokio::main]

async fn main() {
    let state = AppState {
        todos: Arc::new(Mutex::new(Vec::new())),
    };
    let app = Router::new()
        .route("/todo", get(get_todos).post(add_todo))
        .route("/todo/{id}/complete", patch(complete_todo))
        .route("/todo/{id}", get(get_todo_by_id).delete(delete_todo))
        .with_state(state);

    println!("Listening on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

