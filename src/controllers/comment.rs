use crate::db;
use crate::models::{Comment, CommentForm, CommentParams, PageParams};
use crate::traits::CRUD;
use crate::util::EndpointResult;
use actix_web::{web, HttpResponse};

pub async fn comment_list(params: web::Query<PageParams>) -> EndpointResult {
    // TODO - get comment list given url (domain with slug, e.g kaviraj.in/to-kamao-bsdk)..
    let connection = db::get_db_connection();

    let result = Comment::list(&connection, params.id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_comment(params: web::Query<CommentParams>) -> EndpointResult {
    // given comment_id return comment
    let connection = db::get_db_connection();

    let result = Comment::read(&connection, params.id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn comment_create(comment_form: web::Json<CommentForm>) -> EndpointResult {
    // TODO - get `user_id` from session cookie..
    // ensure page_id, parent_id exists

    let connection = db::get_db_connection();

    let result: Comment = Comment::create(&connection, &comment_form)?;

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

    use crate::api_routes;
    use crate::models::{Comment, Page, User};
    use crate::test::fixtures::{comment_1, page_1, user_2};
    use crate::test::transaction;
    use crate::test::Transaction;

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_comment_list(_transaction: Transaction, comment_1: Comment) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let resp = TestRequest::get()
            .uri(&format!("/comments?id={}", comment_1.page_id))
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
    async fn test_comment_create(_transaction: Transaction, page_1: Page, user_2: User) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let req = json!({
            "user_id": user_2.id,
            "page_id": page_1.id,
            "content": String::from("behtareen"),
        });

        let resp = TestRequest::post()
            .uri("/comment")
            .set_json(&req)
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to create comment");
        let comment: Comment = test::read_body_json(resp).await;
        assert_eq!(*req.get("user_id").unwrap(), comment.user_id);
        assert_eq!(*req.get("page_id").unwrap(), comment.page_id);
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
