use diesel::{Insertable, AsChangeset};
use actix_web::{web, Responder, HttpResponse};
use diesel::{prelude::*, insert_into};
use crate::schema::{comment};
use crate::models::{Comment};
use serde::{Deserialize};
use crate::db;


pub async fn comment_list() -> impl Responder {
    // TODO given app_id and page_id returns all comments
    let connection = db::get_db_connection();
    
    let results = comment::table
        .load::<Comment>(&connection)
        .expect("Error loading tests");

    println!("Got {} results", results.len());

    web::Json(results)
}

#[derive(Deserialize)]
pub struct CommentParams {
    id: i32,
}

pub async fn get_comment(params: web::Query<CommentParams>) -> impl Responder {
    // given comment_id return comment
    let connection = db::get_db_connection();

    let result = comment::table.filter(comment::id.eq(params.id))
        .first::<Comment>(&connection)
        .expect("Error getting comment");
    
    web::Json(result)
}


#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="comment"]
pub struct CommentForm {
  pub created_by: i32,
  pub page_id: i32,
  pub parent_id: Option<i32>,
  pub content: String,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub updated_at: Option<chrono::NaiveDateTime>,
}


pub async fn comment_create(mut comment_form: web::Json<CommentForm>) -> impl Responder {
    // TODO - get `created_by` from session cookie..
    // ensure page_id, parent_id exists

    let connection = db::get_db_connection();

    comment_form.created_at = Some(chrono::prelude::Utc::now().naive_utc());
    comment_form.updated_at = Some(chrono::prelude::Utc::now().naive_utc());

    let result: Comment = insert_into(comment::table)
        .values(&*comment_form)
        .get_result(&connection)
        .expect("Error in inserting comment");

    web::Json(result)
}

pub async fn comment_update(params: web::Query<CommentParams>,
    comment_form: web::Json<CommentForm>) -> impl Responder {

    let connection = db::get_db_connection();

    let comment = diesel::update(comment::table.find(params.id))
        .set(&*comment_form)
        .get_result::<Comment>(&connection)
        .expect(&format!("Unable to find comment with {}", params.id));
    println!("Updated comment {}", comment.id);

    web::Json(comment)
}

pub async fn comment_delete(params: web::Query<CommentParams>) -> impl Responder {
    // TODO - throw error when comment not found!!!
    let connection = db::get_db_connection();

    let result = comment::table.filter(comment::id.eq(params.id));    
    let result = diesel::delete(result)
        .execute(&connection)
        .expect("Can't delete comment");

    HttpResponse::Ok().json(result)
}
