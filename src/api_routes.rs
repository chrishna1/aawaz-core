use crate::controllers::{app::*, comment::*, page::*, user::*};
use actix_files::Files;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/api/v1")
                    .route("/comments", web::get().to(comment_list))
                    .route("/comment", web::get().to(get_comment))
                    .route("/comment", web::put().to(comment_create))
                    .route("/comment", web::patch().to(comment_update))
                    .route("/comment", web::delete().to(comment_delete))
                    // page relbuild_routerated routes
                    .route("/pages", web::get().to(page_list))
                    .route("/page", web::get().to(get_page))
                    .route("/page", web::put().to(page_create))
                    .route("/page", web::patch().to(page_update))
                    .route("/page", web::delete().to(page_delete))
                    // app related routes
                    .route("/app", web::get().to(get_app))
                    .route("/app", web::put().to(app_create))
                    .route("/app", web::patch().to(app_update))
                    .route("/app", web::delete().to(app_delete))
                    // user related routes
                    .route("/user", web::get().to(get_user))
                    .route("/user", web::put().to(user_create))
                    .route("/user", web::patch().to(user_update))
                    .route("/user", web::delete().to(user_delete)),
            )
            .service(
                web::scope("")
                    // serve static files!!
                    .service(Files::new("/", "./static/").index_file("index.html")),
            ),
    );
}
