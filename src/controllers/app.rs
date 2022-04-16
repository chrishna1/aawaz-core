use diesel::{Insertable, AsChangeset};
use actix_web::{web, Responder, HttpResponse};
use diesel::{prelude::*, insert_into};
use crate::schema::{app};
use crate::models::{App};
use serde::{Deserialize};
use crate::controllers::util::establish_connection;


#[derive(Deserialize)]
pub struct AppParams {
    id: i32,
}

pub async fn get_app(params: web::Query<AppParams>) -> impl Responder {
    // given page_id return app
    let connection = establish_connection();

    let result = app::table.filter(app::id.eq(params.id))
        .first::<App>(&connection)
        .expect("Error getting page");
    
    web::Json(result)
}


#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="app"]
pub struct AppForm {
  pub name: String,
  pub domain: String,
  pub owner: i32,
}


pub async fn app_create(app_form: web::Json<AppForm>) -> impl Responder {
    // TODO - get `created_by` from session cookie..
    // ensure owner exists
    // validate domain, ensure it's a valid domain

    let connection = establish_connection();

    let result: App = insert_into(app::table)
        .values(&*app_form)
        .get_result(&connection)
        .expect("Error in creating app");

    web::Json(result)
}

pub async fn app_update(params: web::Query<AppParams>,
    app_form: web::Json<AppForm>) -> impl Responder {

    // TODO - get `created_by` from session cookie..
    // ensure owner exists
    // validate domain, ensure it's a valid domain

    let connection = establish_connection();

    let app = diesel::update(app::table.find(params.id))
        .set(&*app_form)
        .get_result::<App>(&connection)
        .expect(&format!("Unable to find app with {}", params.id));
    println!("Updated app {}", app.id);

    web::Json(app)
}

pub async fn app_delete(params: web::Query<AppParams>) -> impl Responder {
    // TODO - throw error when app not found!!!
    let connection = establish_connection();

    let result = app::table.filter(app::id.eq(params.id));    
    let result = diesel::delete(result)
        .execute(&connection)
        .expect("Can't delete app");

    HttpResponse::Ok().json(result)
}
