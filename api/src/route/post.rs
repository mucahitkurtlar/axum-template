use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use entity::post::Model as PostModel;
use service::dto::post::{CreatePostRequest, UpdatePostRequest};
use service::post;

use crate::AppState;

pub async fn create(
    state: State<AppState>,
    json_data: Json<CreatePostRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    match post::create(&state.conn, json_data.0).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<PostModel>, (StatusCode, String)> {
    match post::get(&state.conn, id).await {
        Ok(post) => {
            if let Some(post) = post {
                Ok(Json(post))
            } else {
                Err((StatusCode::NOT_FOUND, "Post not found".to_string()))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update(
    state: State<AppState>,
    Path(id): Path<i32>,
    json_data: Json<UpdatePostRequest>,
) -> Result<Json<PostModel>, (StatusCode, String)> {
    match post::update(&state.conn, id, json_data.0).await {
        Ok(post) => Ok(Json(post)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    match post::delete(&state.conn, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
