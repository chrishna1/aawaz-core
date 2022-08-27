use crate::db;
use crate::models::{AppParams, Page, PageForm};
use crate::traits::CRUD;
use crate::util::EndpointResult;
use actix_web::{web, HttpResponse};

pub async fn page_list(params: web::Query<AppParams>) -> EndpointResult {
    // given app_id returns all pages associated with the app.
    let connection = db::get_db_connection();

    let result = Page::list(&connection, params.id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn page_create(page_form: web::Json<PageForm>) -> EndpointResult {
    // TODO - get `user_id` from session cookie..
    // ensure app_id exists
    // sanitize path, ensure it's valid path

    let connection = db::get_db_connection();

    let result: Page = Page::create(&connection, &page_form)?;

    Ok(HttpResponse::Ok().json(result))
}

#[cfg(test)]
mod tests {
    use crate::api_routes;
    use crate::models::{App, Page};
    use crate::test::fixtures::{app_1, page_1};
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
    async fn test_page_create(_transaction: Transaction, app_1: App) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let payload = json!({
            "app_id": app_1.id,
            "path": "/zindagi-kaisi-hai-paheli"
        });

        let resp = TestRequest::post()
            .uri("/page")
            .set_json(&payload)
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to create page");
        let _page: Page = test::read_body_json(resp).await;
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_page_list(_transaction: Transaction, page_1: Page) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let resp = TestRequest::get()
            .uri(&format!("/pages?id={}", page_1.app_id))
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to get list of pages");

        let pages: Vec<Page> = test::read_body_json(resp).await;
        assert_eq!(pages.len(), 1);
        assert_eq!(page_1, pages[0]);

        // TODO.. test with multiple pages
    }
}
