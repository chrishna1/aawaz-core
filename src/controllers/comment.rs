use crate::db;
use crate::models::{Comment, CommentForm};
use crate::traits::CRUD;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub async fn comment_list() -> impl Responder {
    let connection = db::get_db_connection();

    let results = Comment::list(&connection, 1) // TODO - send page_id
        .expect("Error loading tests");

    web::Json(results)
}

#[derive(Deserialize)]
pub struct CommentParams {
    id: i32,
}

pub async fn get_comment(params: web::Query<CommentParams>) -> impl Responder {
    // given comment_id return comment
    let connection = db::get_db_connection();

    let result = Comment::read(&connection, params.id).expect("Error getting comment");

    web::Json(result)
}

pub async fn comment_create(comment_form: web::Json<CommentForm>) -> impl Responder {
    // TODO - get `user_id` from session cookie..
    // ensure page_id, parent_id exists

    let connection = db::get_db_connection();

    let result: Comment =
        Comment::create(&connection, &comment_form).expect("Error in creating comment");

    web::Json(result)
}

pub async fn comment_update(
    params: web::Query<CommentParams>,
    comment_form: web::Json<CommentForm>,
) -> impl Responder {
    let connection = db::get_db_connection();

    let comment =
        Comment::update(&connection, params.id, &comment_form).expect("Error in updating comment");

    web::Json(comment)
}

pub async fn comment_delete(params: web::Query<CommentParams>) -> impl Responder {
    // TODO - throw error when comment not found!!!
    let connection = db::get_db_connection();

    let result = Comment::delete(&connection, params.id).expect("Error in deleting comment");

    HttpResponse::Ok().json(result)
}
