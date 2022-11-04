use crate::auth::github::{callback, login};
use crate::auth::logout::logout;
use crate::controllers::{app::*, comment::*, page::*, user::*};
use actix_files::Files;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig, api_ver: &str) {
    cfg.service(
        web::scope("")
            .service(
                web::scope(api_ver)
                    .route("/comments", web::get().to(comment_list))
                    .route("/comment", web::get().to(get_comment))
                    .route("/comment", web::post().to(comment_create))
                    .route("/comment", web::put().to(comment_update))
                    .route("/comment", web::delete().to(comment_delete))
                    // page related routes
                    .route("/pages", web::get().to(page_list))
                    .route("/page", web::post().to(page_create))
                    // app related routes
                    .route("/app", web::get().to(get_app))
                    .route("/app", web::post().to(app_create))
                    .route("/app", web::put().to(app_update))
                    .route("/app", web::delete().to(app_delete))
                    // user related routes
                    .route("/user", web::get().to(get_user))
                    .route("/user", web::post().to(user_create))
                    .route("/user", web::put().to(user_update))
                    .route("/user", web::delete().to(user_delete)),
            )
            .service(
                web::scope("")
                    // serve static files!!
                    .route("/auth/github", web::get().to(login))
                    .route("/auth/github/callback", web::get().to(callback))
                    .route("/logout", web::get().to(logout))
                    .service(Files::new("/", "./static/").index_file("index.html")),
            ),
    );
}
