use axum::{
    routing::{get, patch, delete},
    Router,
};

use sqlx::SqlitePool;

mod models;
mod routes;
mod state;
mod db;

use state::AppState;
use routes::todo::{
    get_todos,
    get_todo_by_id,
    add_todo,
    complete_todo,
    delete_todo,
};

#[tokio::main]
async fn main() {
   
    dotenvy::dotenv().ok();

    
    let pool: SqlitePool = db::connect_db().await;

    
    let state = AppState {
        db_pool: pool,
    };

    
    let app = Router::new()
        .route("/todo", get(get_todos).post(add_todo))
        .route("/todo/{id}", get(get_todo_by_id).delete(delete_todo))
        .route("/todo/{id}/complete", patch(complete_todo))
        .with_state(state);

    println!("Server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
