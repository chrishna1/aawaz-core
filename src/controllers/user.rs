use crate::db;
use crate::models::{User, UserForm, UserParams};
use crate::traits::CRUD;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_user(params: web::Query<UserParams>) -> impl Responder {
    // given user_id return user
    let connection = db::get_db_connection();

    let result = User::read(&connection, params.id).expect("Error getting user");

    web::Json(result)
}

pub async fn user_create(user_form: web::Json<UserForm>) -> impl Responder {
    // TODO - hash password
    // ensure username is available?? - websocket???
    // ensure email exists???
    // send verification email

    let connection = db::get_db_connection();

    let result: User = User::create(&connection, &user_form).expect("Error in creating user");

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

    let user = User::update(&connection, params.id, &user_form)
        .expect(&format!("Unable to find user with {}", params.id));

    web::Json(user)
}

pub async fn user_delete(params: web::Query<UserParams>) -> impl Responder {
    // TODO - throw error when user not found!!!
    // username all the comments of the user..
    let connection = db::get_db_connection();

    let result = User::delete(&connection, params.id).expect("Can't delete user");

    HttpResponse::Ok().json(result)
}
