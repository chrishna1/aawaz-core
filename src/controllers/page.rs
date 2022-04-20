use crate::db;
use crate::models::Page;
use crate::schema::page;
use actix_web::{web, HttpResponse, Responder};
use diesel::{insert_into, prelude::*};
use diesel::{AsChangeset, Insertable};
use serde::Deserialize;

pub async fn page_list() -> impl Responder {
    // given app_id returns all pages associated with the app.
    let connection = db::get_db_connection();

    let results = page::table
        .load::<Page>(&connection)
        .expect("Error loading pages");

    println!("Got {} results", results.len());

    web::Json(results)
}

#[derive(Deserialize)]
pub struct PageParams {
    id: i32,
}

pub async fn get_page(params: web::Query<PageParams>) -> impl Responder {
    // given page_id return page
    let connection = db::get_db_connection();

    let result = page::table
        .filter(page::id.eq(params.id))
        .first::<Page>(&connection)
        .expect("Error getting page");

    web::Json(result)
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "page"]
pub struct PageForm {
    pub app_id: i32,
    pub slug: String,
}

pub async fn page_create(page_form: web::Json<PageForm>) -> impl Responder {
    // TODO - get `created_by` from session cookie..
    // ensure app_id exists
    // sanitize slug, ensure it's valid slug

    let connection = db::get_db_connection();

    let result: Page = insert_into(page::table)
        .values(&*page_form)
        .get_result(&connection)
        .expect("Error in creating page");

    web::Json(result)
}

pub async fn page_update(
    params: web::Query<PageParams>,
    page_form: web::Json<PageForm>,
) -> impl Responder {
    // TODO - get `created_by` from session cookie..
    // ensure app_id exists
    // sanitize slug, ensure it's valid slug

    let connection = db::get_db_connection();

    let page = diesel::update(page::table.find(params.id))
        .set(&*page_form)
        .get_result::<Page>(&connection)
        .expect(&format!("Unable to find page with {}", params.id));
    println!("Updated page {}", page.id);

    web::Json(page)
}

pub async fn page_delete(params: web::Query<PageParams>) -> impl Responder {
    // TODO - throw error when page not found!!!
    let connection = db::get_db_connection();

    let result = page::table.filter(page::id.eq(params.id));
    let result = diesel::delete(result)
        .execute(&connection)
        .expect("Can't delete page");

    HttpResponse::Ok().json(result)
}
