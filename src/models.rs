// src/models.rs
use diesel::prelude::*;
use crate::schema::{users, posts};
use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: Option<String>,
    pub password: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub author_id: uuid::Uuid,
}
