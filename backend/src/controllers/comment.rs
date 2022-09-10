use crate::db;
use crate::models::{
    App, Comment, CommentForm, CommentParams, CommentPayload, Page, PageForm, PagePayload,
};
use crate::traits::CRUD;
use crate::util::EndpointResult;
use actix_web::{web, HttpResponse};
use url::Url;

pub async fn comment_list(pp: web::Query<PagePayload>) -> EndpointResult {
    // TODO - get comment list given url (domain with path, e.g kaviraj.in/to-kamao-bsdk)..
    let connection = db::get_db_connection();

    // get page id from path..
    // get app_id from domain
    // get page_id from app_id and path
    // if it doesn't exist, it might be no one has commented on this page yet. return empty error in this case

    let parsed_url = Url::parse(&pp.url)?;

    let domain = parsed_url.origin().unicode_serialization();

    let path = parsed_url.path();

    // get app_id from domain
    // get page_id from app_id + path
    // if page_id exist ok, else create page if it doesn't exist(first comment on a page)!!

    println!("domain: {}", domain);
    let app = App::from_domain(&connection, domain)?;

    let page = match Page::from_app_id_and_path(&connection, app.id, String::from(path)) {
        Ok(page) => page,
        Err(_) => {
            let comments: Vec<Comment> = vec![];
            return Ok(HttpResponse::Ok().json(comments));
        }
    };

    let result = Comment::list(&connection, page.id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_comment(params: web::Query<CommentParams>) -> EndpointResult {
    // given comment_id return comment
    let connection = db::get_db_connection();

    let result = Comment::read(&connection, params.id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn comment_create(comment_form: web::Json<CommentPayload>) -> EndpointResult {
    // TODO - get `user_id` from session cookie..
    // ensure page_id, parent_id exists
    let user_id = 1;

    let connection = db::get_db_connection();

    let parsed_url = Url::parse(&comment_form.url)?;

    let domain = parsed_url.origin().unicode_serialization();

    let path = parsed_url.path();

    // get app_id from domain
    // get page_id from app_id + path
    // if page_id exist ok, else create page if it doesn't exist(first comment on a page)!!

    println!("domain: {}", domain);
    let app = App::from_domain(&connection, domain)?;

    let page = match Page::from_app_id_and_path(&connection, app.id, String::from(path)) {
        Ok(page) => page,
        Err(_) => Page::create(
            &connection,
            &PageForm {
                app_id: app.id,
                path: String::from(path),
            },
        )?,
    };

    let comment = CommentForm {
        user_id,
        page_id: page.id,
        parent_id: comment_form.parent_id,
        content: comment_form.content.clone(), // TODO - any better way??
    };
    let result: Comment = Comment::create(&connection, &comment)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn comment_update(
    params: web::Query<CommentParams>,
    comment_form: web::Json<CommentForm>,
) -> EndpointResult {
    let connection = db::get_db_connection();

    let result = Comment::update(&connection, params.id, &comment_form)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn comment_delete(params: web::Query<CommentParams>) -> EndpointResult {
    let connection = db::get_db_connection();

    Comment::delete(&connection, params.id)?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{
        test::{self, TestRequest},
        App as ActixApp,
    };
    use rstest::rstest;
    use serde_json::json;

    use crate::models::{Comment, Page};
    use crate::test::fixtures::{comment_1, page_1};
    use crate::test::transaction;
    use crate::test::Transaction;
    use crate::traits::CRUD;
    use crate::{api_routes, db};

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_comment_list(_transaction: Transaction, comment_1: Comment) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let url;
        {
            let conn = db::get_db_connection();
            let page_1 = Page::read(&conn, comment_1.page_id).unwrap();
            url = page_1.get_url(&conn).unwrap();
        }

        let resp = TestRequest::get()
            .uri(&format!("/comments?url={}", url))
            .send_request(&mut service)
            .await;

        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "Failed to get list of comments"
        );

        let comments: Vec<Comment> = test::read_body_json(resp).await;
        assert_eq!(comments.len(), 1);
        assert_eq!(comment_1, comments[0]);
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_comment_create(_transaction: Transaction, page_1: Page) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let url;
        {
            let conn = db::get_db_connection();
            url = page_1.get_url(&conn).unwrap();
        }

        let req = json!({
            "url": url,
            "content": String::from("behtareen"),
        });

        let resp = TestRequest::post()
            .uri("/comment")
            .set_json(&req)
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to create comment");
        let comment: Comment = test::read_body_json(resp).await;
        assert_eq!(*req.get("content").unwrap(), comment.content);
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_comment_update(_transaction: Transaction, comment_1: Comment) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let req = json!({
            "user_id": comment_1.user_id,
            "page_id": comment_1.page_id,
            "content": comment_1.content + "updated"
        });

        let resp = TestRequest::put()
            .uri(&format!("/comment?id={}", comment_1.id))
            .set_json(&req)
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to update comment");
        let comment: Comment = test::read_body_json(resp).await;
        assert_eq!(*req.get("user_id").unwrap(), comment.user_id);
        assert_eq!(*req.get("page_id").unwrap(), comment.page_id);
        assert_eq!(*req.get("content").unwrap(), comment.content);
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_comment_delete(_transaction: Transaction, comment_1: Comment) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let resp = TestRequest::delete()
            .uri(&format!("/comment?id={}", comment_1.id))
            .send_request(&mut service)
            .await;

        assert_eq!(
            resp.status(),
            StatusCode::NO_CONTENT,
            "Failed to delete comment"
        );
    }
}
