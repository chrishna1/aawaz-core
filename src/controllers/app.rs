use crate::db;
use crate::models::{App, AppForm};
use crate::traits::CRUD;
use crate::util::EndpointResult;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppParams {
    id: i32,
}

pub async fn get_app(params: web::Query<AppParams>) -> EndpointResult {
    let connection = db::get_db_connection();

    let result = App::read(&connection, params.id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn app_create(app_form: web::Json<AppForm>) -> EndpointResult {
    // TODO - get `user_id` from session cookie..
    // ensure owner exists
    // ensure it's a valid domain

    let connection = db::get_db_connection();

    let result = App::create(&connection, &app_form)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn app_update(
    params: web::Query<AppParams>,
    app_form: web::Json<AppForm>,
) -> EndpointResult {
    // TODO - get `user_id` from session cookie..
    // ensure owner exists
    // validate domain, ensure it's a valid domain

    let connection = db::get_db_connection();

    let result = App::update(&connection, params.id, &app_form)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn app_delete(params: web::Query<AppParams>) -> EndpointResult {
    // TODO - throw error when app not found!!!
    let connection = db::get_db_connection();

    App::delete(&connection, params.id)?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use crate::api_routes;
    use crate::models::{App, User};
    use crate::test::fixtures::{app_1, user_1};
    use crate::test::{transaction, Transaction};
    use actix_web::http::StatusCode;
    use actix_web::{
        test::{self, TestRequest},
        App as ActixApp,
    };
    use rstest::*;
    use serde_json::json;

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_app_create(_transaction: Transaction, user_1: User) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        // test create app
        let request_body_app = json!({
            "name": "muntasir blog\"s comment",
            "domain": "https://muntasir.com",
            "owner": user_1.id
        });

        let resp = TestRequest::put()
            .uri("/app")
            .set_json(&request_body_app)
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to create app");
        let _app: App = test::read_body_json(resp).await;
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_app_get(_transaction: Transaction, app_1: App) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        // test get app
        let resp = TestRequest::get()
            .uri(&format!("/app?id={}", app_1.id))
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to get app");

        let app: App = test::read_body_json(resp).await;
        assert_eq!(app_1.name, app.name);
        assert_eq!(app_1.domain, app.domain);
        assert_eq!(app_1.owner, app.owner);
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_app_update(_transaction: Transaction, app_1: App) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        // test update app
        let req = json!({
            "name": app_1.name + "updated",
            "domain": app_1.domain + "updated",
            "owner": app_1.owner
        });

        let resp = TestRequest::patch()
            .uri(&format!("/app?id={}", app_1.id))
            .set_json(&req)
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to update app");
        let app: App = test::read_body_json(resp).await;
        assert_eq!(*req.get("name").unwrap(), app.name);
        assert_eq!(*req.get("domain").unwrap(), app.domain);
        assert_eq!(*req.get("owner").unwrap(), app.owner);
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_app_delete(_transaction: Transaction, app_1: App) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        // test delete app
        let resp = TestRequest::delete()
            .uri(&format!("/app?id={}", app_1.id))
            .send_request(&mut service)
            .await;

        assert_eq!(
            resp.status(),
            StatusCode::NO_CONTENT,
            "Failed to delete app"
        );
    }
}
