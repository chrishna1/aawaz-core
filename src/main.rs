use std::env;
use diesel::{pg::PgConnection};
use actix_web::{web, App as ActixWebApp, HttpServer};
use actix_files::{Files};
use dotenv::dotenv;
use diesel::{prelude::*};

use aawaz::controllers::{comment::*, app::*, page::*, user::*};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[rustfmt::skip]
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {

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

        .service(Files::new("/", "./static/").index_file("index.html"))


    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
