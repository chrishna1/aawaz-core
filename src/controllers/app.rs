use crate::db;
use crate::models::{App, AppForm};
use crate::traits::CRUD;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppParams {
    id: i32,
}

pub async fn get_app(params: web::Query<AppParams>) -> impl Responder {
    let connection = db::get_db_connection();

    let result = App::read(&connection, params.id).expect("App not found");

    web::Json(result)
}

pub async fn app_create(app_form: web::Json<AppForm>) -> impl Responder {
    // TODO - get `user_id` from session cookie..
    // ensure owner exists
    // ensure it's a valid domain

    let connection = db::get_db_connection();

    let result = App::create(&connection, &app_form).expect("Error in creating app");

    web::Json(result)
}

pub async fn app_update(
    params: web::Query<AppParams>,
    app_form: web::Json<AppForm>,
) -> impl Responder {
    // TODO - get `user_id` from session cookie..
    // ensure owner exists
    // validate domain, ensure it's a valid domain

    let connection = db::get_db_connection();

    let app = App::update(&connection, params.id, &app_form).expect("Error in updating app");

    web::Json(app)
}

pub async fn app_delete(params: web::Query<AppParams>) -> impl Responder {
    // TODO - throw error when app not found!!!
    let connection = db::get_db_connection();

    let result = App::delete(&connection, params.id).expect("Error in deleting app");

    HttpResponse::Ok().json(result)
}
