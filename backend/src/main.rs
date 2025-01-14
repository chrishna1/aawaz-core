use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{self, Key, SameSite},
    App as ActixWebApp, HttpServer,
};
use dotenv::dotenv;
use std::env;

use aawaz::{api_routes, db};

#[rustfmt::skip]
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();

    db::init();

    HttpServer::new(|| {
        ActixWebApp::new()
            .wrap(
                // TODO - restrict it to only supported methods, headers, etc.
                Cors::permissive()
            )
            // cookie session middleware
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_name(env::var("COOKIE_NAME").expect("COOKIE_NAME is required").to_owned())
                    .cookie_secure(false)
                    .cookie_same_site(SameSite::None)
                    .cookie_secure(true)
                    // customize session and cookie expiration
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::days(30)),
                    )
                    .build(),
            )
            .configure(|cfg| api_routes::config(cfg, "/api/v1"))
    })
    .bind(("localhost", 8080))? // TODO - pick this from env variable
    .run()
    .await
}
