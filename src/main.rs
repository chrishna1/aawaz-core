use std::env;
use std::path::PathBuf;
use diesel::{pg::PgConnection, Insertable, AsChangeset};
use actix_web::{web, App as ActixWebApp, HttpServer, Responder, Result, HttpRequest, HttpResponse};
use actix_files::{NamedFile, Files};
use dotenv::dotenv;
use diesel::{prelude::*, insert_into};

use aawaz::schema::{comment, page, app, users};
use aawaz::models::{Comment, Page, App, User};
use serde::{Deserialize};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn index() -> &'static str {
    "Welcome!"
}

async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    println!("path requested: {:?}", path);
    Ok(NamedFile::open(path)?)
}

#[derive(Deserialize)]
pub struct UserParams {
    id: i32,
}

async fn get_user(params: web::Query<UserParams>) -> impl Responder {
    // given user_id return user
    let connection = establish_connection();

    let result = users::table.filter(users::id.eq(params.id))
        .first::<User>(&connection)
        .expect("Error getting user");
    
    web::Json(result)
}


#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="users"]
pub struct UserForm {
  pub handle: String,
  pub name: String,
  pub email: String,
  pub password: String,
}


async fn user_create(user_form: web::Json<UserForm>) -> impl Responder {
    // TODO - hash password
    // ensure handle is available?? - websocket???
    // ensure email exists???
    // send verification email

    let connection = establish_connection();

    let result: User = insert_into(users::table)
        .values(&*user_form)
        .get_result(&connection)
        .expect("Error in creating user");

    web::Json(result)
}

async fn user_update(params: web::Query<UserParams>,
    user_form: web::Json<UserForm>) -> impl Responder {

    // TODO - get `created_by` from session cookie..
    // ensure user exists
    // if new email, ensure email exists???
    // send verification for the email

    let connection = establish_connection();

    let user = diesel::update(users::table.find(params.id))
        .set(&*user_form)
        .get_result::<User>(&connection)
        .expect(&format!("Unable to find user with {}", params.id));
    println!("Updated user {}", user.id);

    web::Json(user)
}

async fn user_delete(params: web::Query<UserParams>) -> impl Responder {
    // TODO - throw error when user not found!!!
    // handle all the comments of the user.. 
    let connection = establish_connection();

    let result = users::table.filter(users::id.eq(params.id));    
    let result = diesel::delete(result)
        .execute(&connection)
        .expect("Can't delete user");

    HttpResponse::Ok().json(result)
}




#[derive(Deserialize)]
pub struct AppParams {
    id: i32,
}

async fn get_app(params: web::Query<AppParams>) -> impl Responder {
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


async fn app_create(app_form: web::Json<AppForm>) -> impl Responder {
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

async fn app_update(params: web::Query<CommentParams>,
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

async fn app_delete(params: web::Query<AppParams>) -> impl Responder {
    // TODO - throw error when app not found!!!
    let connection = establish_connection();

    let result = app::table.filter(app::id.eq(params.id));    
    let result = diesel::delete(result)
        .execute(&connection)
        .expect("Can't delete app");

    HttpResponse::Ok().json(result)
}

async fn page_list() -> impl Responder {
    // given app_id returns all pages associated with the app.
    let connection = establish_connection();
    
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

async fn get_page(params: web::Query<PageParams>) -> impl Responder {
    // given page_id return page
    let connection = establish_connection();

    let result = page::table.filter(page::id.eq(params.id))
        .first::<Page>(&connection)
        .expect("Error getting page");
    
    web::Json(result)
}


#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="page"]
pub struct PageForm {
  pub app_id: i32,
  pub slug: String,
}


async fn page_create(page_form: web::Json<PageForm>) -> impl Responder {
    // TODO - get `created_by` from session cookie..
    // ensure app_id exists
    // sanitize slug, ensure it's valid slug

    let connection = establish_connection();

    let result: Page = insert_into(page::table)
        .values(&*page_form)
        .get_result(&connection)
        .expect("Error in creating page");

    web::Json(result)
}

async fn page_update(params: web::Query<CommentParams>,
    page_form: web::Json<PageForm>) -> impl Responder {

    // TODO - get `created_by` from session cookie..
    // ensure app_id exists
    // sanitize slug, ensure it's valid slug

    let connection = establish_connection();

    let page = diesel::update(page::table.find(params.id))
        .set(&*page_form)
        .get_result::<Page>(&connection)
        .expect(&format!("Unable to find page with {}", params.id));
    println!("Updated page {}", page.id);

    web::Json(page)
}

async fn page_delete(params: web::Query<PageParams>) -> impl Responder {
    // TODO - throw error when page not found!!!
    let connection = establish_connection();

    let result = page::table.filter(page::id.eq(params.id));    
    let result = diesel::delete(result)
        .execute(&connection)
        .expect("Can't delete page");

    HttpResponse::Ok().json(result)
}
async fn comment_list() -> impl Responder {
    // TODO given app_id and page_id returns all comments
    let connection = establish_connection();
    
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

async fn get_comment(params: web::Query<CommentParams>) -> impl Responder {
    // given comment_id return comment
    let connection = establish_connection();

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
}


async fn comment_create(comment_form: web::Json<CommentForm>) -> impl Responder {
    // TODO - get `created_by` from session cookie..
    // ensure page_id, parent_id exists

    let connection = establish_connection();

    let result: Comment = insert_into(comment::table)
        .values(&*comment_form)
        .get_result(&connection)
        .expect("Error in inserting comment");

    web::Json(result)
}

async fn comment_update(params: web::Query<CommentParams>,
    comment_form: web::Json<CommentForm>) -> impl Responder {

    let connection = establish_connection();

    let comment = diesel::update(comment::table.find(params.id))
        .set(&*comment_form)
        .get_result::<Comment>(&connection)
        .expect(&format!("Unable to find comment with {}", params.id));
    println!("Updated comment {}", comment.id);

    web::Json(comment)
}

async fn comment_delete(params: web::Query<CommentParams>) -> impl Responder {
    // TODO - throw error when comment not found!!!
    let connection = establish_connection();

    let result = comment::table.filter(comment::id.eq(params.id));    
    let result = diesel::delete(result)
        .execute(&connection)
        .expect("Can't delete comment");

    HttpResponse::Ok().json(result)
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        ActixWebApp::new()
        // .route("/", web::get().to(index))
        .route("/api/comments", web::get().to(comment_list))
        .route("/api/comment", web::get().to(get_comment))
        .route("/api/comment", web::put().to(comment_create))
        .route("/api/comment", web::patch().to(comment_update))
        .route("/api/comment", web::delete().to(comment_delete))
        // page related routes 
        .route("/api/pages", web::get().to(page_list))
        .route("/api/page", web::get().to(get_page))
        .route("/api/page", web::put().to(page_create))
        .route("/api/page", web::patch().to(page_update))
        .route("/api/page", web::delete().to(page_delete))
        // app related routes 
        .route("/api/app", web::get().to(get_app))
        .route("/api/app", web::put().to(app_create))
        .route("/api/app", web::patch().to(app_update))
        .route("/api/app", web::delete().to(app_delete))
        // user related routes 
        .route("/api/user", web::get().to(get_user))
        .route("/api/user", web::put().to(user_create))
        .route("/api/user", web::patch().to(user_update))
        .route("/api/user", web::delete().to(user_delete))
        // static files
        // .route("{filename:/static/.*}", web::get().to(static_files))
        .service(Files::new("/", "./static/").index_file("index.html"))


    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
