use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub text: Option<String>,
}
