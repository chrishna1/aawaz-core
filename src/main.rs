use actix_web::{App as ActixWebApp, HttpServer};
use diesel::pg::PgConnection;
use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

use aawaz::api_routes;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[rustfmt::skip]
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        ActixWebApp::new()
            .configure(|cfg| api_routes::config(cfg))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
