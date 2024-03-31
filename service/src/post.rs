use ::entity::post::{self as post_entity, Entity as Post};
use sea_orm::*;

use crate::dto::post::{CreatePostRequest, UpdatePostRequest};

pub async fn create(
    db: &DbConn,
    request: CreatePostRequest,
) -> Result<post_entity::ActiveModel, DbErr> {
    post_entity::ActiveModel {
        title: Set(request.title.to_owned()),
        text: Set(request.text.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn get(db: &DbConn, id: i32) -> Result<Option<post_entity::Model>, DbErr> {
    Post::find_by_id(id).one(db).await
}

pub async fn update(
    db: &DbConn,
    id: i32,
    request: UpdatePostRequest,
) -> Result<post_entity::Model, DbErr> {
    let post: post_entity::ActiveModel = Post::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find post_.".to_owned()))
        .map(Into::into)?;

    post_entity::ActiveModel {
        id: post.id,
        title: request.title.map_or(NotSet, Set),
        text: request.text.map_or(NotSet, Set),
    }
    .update(db)
    .await
}

pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    let post: post_entity::ActiveModel = Post::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find post_.".to_owned()))
        .map(Into::into)?;

    post.delete(db).await
}

pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
    Post::delete_many().exec(db).await
}
