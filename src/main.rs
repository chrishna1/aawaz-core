use actix_web::{App as ActixWebApp, HttpServer};

use aawaz::api_routes;

#[rustfmt::skip]
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        ActixWebApp::new()
            .configure(|cfg| api_routes::config(cfg, "/api/v1"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
