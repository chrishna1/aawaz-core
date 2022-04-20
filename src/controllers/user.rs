use crate::db;
use crate::models::User;
use crate::schema::users;
use actix_web::{web, HttpResponse, Responder};
use diesel::{insert_into, prelude::*};
use diesel::{AsChangeset, Insertable};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserParams {
    id: i32,
}

pub async fn get_user(params: web::Query<UserParams>) -> impl Responder {
    // given user_id return user
    let connection = db::get_db_connection();

    let result = users::table
        .filter(users::id.eq(params.id))
        .first::<User>(&connection)
        .expect("Error getting user");

    web::Json(result)
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UserForm {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn user_create(user_form: web::Json<UserForm>) -> impl Responder {
    // TODO - hash password
    // ensure username is available?? - websocket???
    // ensure email exists???
    // send verification email

    let connection = db::get_db_connection();

    let result: User = insert_into(users::table)
        .values(&*user_form)
        .get_result(&connection)
        .expect("Error in creating user");

    web::Json(result)
}

pub async fn user_update(
    params: web::Query<UserParams>,
    user_form: web::Json<UserForm>,
) -> impl Responder {
    // TODO - get `user_id` from session cookie..
    // ensure user exists
    // if new email, ensure email exists???
    // send verification for the email

    let connection = db::get_db_connection();

    let user = diesel::update(users::table.find(params.id))
        .set(&*user_form)
        .get_result::<User>(&connection)
        .expect(&format!("Unable to find user with {}", params.id));

    web::Json(user)
}

pub async fn user_delete(params: web::Query<UserParams>) -> impl Responder {
    // TODO - throw error when user not found!!!
    // username all the comments of the user..
    let connection = db::get_db_connection();

    let result = users::table.filter(users::id.eq(params.id));
    let result = diesel::delete(result)
        .execute(&connection)
        .expect("Can't delete user");

    HttpResponse::Ok().json(result)
}
