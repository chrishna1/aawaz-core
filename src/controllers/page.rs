use crate::db;
use crate::models::{Page, PageForm};
use crate::traits::CRUD;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub async fn page_list() -> impl Responder {
    // given app_id returns all pages associated with the app.
    let connection = db::get_db_connection();

    let results = Page::list(&connection, 1) // TODO - send app_id
        .expect("Error loading pages");

    web::Json(results)
}

#[derive(Deserialize)]
pub struct PageParams {
    id: i32,
}

pub async fn get_page(params: web::Query<PageParams>) -> impl Responder {
    // given page_id return page
    let connection = db::get_db_connection();

    let result = Page::read(&connection, params.id).expect("Error getting page");

    web::Json(result)
}

pub async fn page_create(page_form: web::Json<PageForm>) -> impl Responder {
    // TODO - get `user_id` from session cookie..
    // ensure app_id exists
    // sanitize slug, ensure it's valid slug

    let connection = db::get_db_connection();

    let result: Page = Page::create(&connection, &page_form).expect("Error in creating page");

    web::Json(result)
}

pub async fn page_update(
    params: web::Query<PageParams>,
    page_form: web::Json<PageForm>,
) -> impl Responder {
    // TODO - get `user_id` from session cookie..
    // ensure app_id exists
    // sanitize slug, ensure it's valid slug

    let connection = db::get_db_connection();

    let page = Page::update(&connection, params.id, &page_form).expect("Error in updating page");

    web::Json(page)
}

pub async fn page_delete(params: web::Query<PageParams>) -> impl Responder {
    // TODO - throw error when page not found!!!
    let connection = db::get_db_connection();

    let result = Page::delete(&connection, params.id).expect("Can't delete page");

    HttpResponse::Ok().json(result)
}
