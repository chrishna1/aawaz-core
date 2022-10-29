use actix_cors::Cors;
use actix_web::{App as ActixWebApp, HttpServer};

use aawaz::api_routes;

#[rustfmt::skip]
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        ActixWebApp::new()
            .wrap(
                // TODO - restrict it to only supported methods, headers, etc.
                Cors::permissive()
            )
            .configure(|cfg| api_routes::config(cfg, "/api/v1"))
    })
    .bind(("127.0.0.1", 8080))? // TODO - pick this from env variable
    .run()
    .await
}
