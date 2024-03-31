use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::AppState;

pub mod hello;
pub mod post;

pub fn build(state: AppState) -> Router {
    let api_router = Router::new()
        .route("/hello", get(hello::hello_world))
        .route("/posts", post(post::create))
        .route("/posts/:id", get(post::get))
        .route("/posts/:id", patch(post::update))
        .route("/posts/:id", delete(post::delete))
        .with_state(state.clone());

    Router::new().nest("/api/v1", api_router).with_state(state)
}
