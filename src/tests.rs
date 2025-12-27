use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::{get, post, delete},
    Router,
};

use tower::util::ServiceExt;


use std::sync::{Arc, Mutex};

use crate::{
    state::AppState,
    routes::todo::{get_todos, add_todo, delete_todo},
};


fn setup_app() -> Router {
    let state = AppState {
        todos: Arc::new(Mutex::new(Vec::new())),
    };

    Router::new()
        .route("/todo", get(get_todos).post(add_todo))
        .route("/todo/{id}", delete(delete_todo))
        .with_state(state)
}


#[tokio::test]
async fn test_add_todo() {
    let app = setup_app();

    let request = Request::builder()
        .method("POST")
        .uri("/todo")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"Test Todo"}"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}


#[tokio::test]
async fn test_get_todos() {
    let app = setup_app();

    let request = Request::builder()
        .uri("/todo")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}


#[tokio::test]
async fn test_delete_todo() {
    let app = setup_app();

    let create_request = Request::builder()
        .method("POST")
        .uri("/todo")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"Delete Me"}"#))
        .unwrap();

    app.clone().oneshot(create_request).await.unwrap();

    let delete_request = Request::builder()
        .method("DELETE")
        .uri("/todo/1")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(delete_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
