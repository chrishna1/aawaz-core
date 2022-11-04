use crate::db;
use crate::models::{User, UserForm, UserParams};
use crate::traits::CRUD;
use crate::util::EndpointResult;
use actix_session::Session;
use actix_web::{web, HttpResponse};

pub async fn get_current_user(session: Session) -> EndpointResult {
    match session.get("uid").unwrap() {
        Some(uid) => {
            let connection = db::get_db_connection();

            let result = User::read(&connection, uid)?;

            return Ok(HttpResponse::Ok().json(result));
        }
        None => return Ok(HttpResponse::Ok().json(None::<bool>)),
    }
}

pub async fn get_user(params: web::Query<UserParams>) -> EndpointResult {
    // given user_id return user
    let connection = db::get_db_connection();

    let result = User::read(&connection, params.id)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn user_create(user_form: web::Json<UserForm>) -> EndpointResult {
    // TODO - hash password
    // ensure username is available?? - websocket???
    // ensure email exists???
    // send verification email

    let connection = db::get_db_connection();

    let result: User = User::create(&connection, &user_form)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn user_update(
    params: web::Query<UserParams>,
    user_form: web::Json<UserForm>,
) -> EndpointResult {
    // TODO - get `user_id` from session cookie..
    // ensure user exists
    // if new email, ensure email exists???
    // send verification for the email

    let connection = db::get_db_connection();

    let result = User::update(&connection, params.id, &user_form)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn user_delete(params: web::Query<UserParams>) -> EndpointResult {
    // A lot TODO
    // handle comments added by user(stackoverflow style?)...
    // handle votes added by user!!
    // app owner can't delete their account without deleting the app first
    let connection = db::get_db_connection();

    User::delete(&connection, params.id)?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use crate::api_routes;
    use crate::models::User;
    use crate::test::fixtures::user_1;
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
    async fn test_user_create(_transaction: Transaction) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let req = json!({
            "username": "muntasir",
            "name": "muntasir",
            "email": "muntasir@muntasir.com",
            "password": "muntasir",
        });

        let resp = TestRequest::post()
            .uri("/user")
            .set_json(&req)
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to create user");
        let _user: User = test::read_body_json(resp).await;
    }

    // TODO - fix this error..
    // #[rstest]
    // #[trace]
    // #[actix_rt::test]
    // async fn test_duplicate_username(_transaction: Transaction, user_1: User) {
    //     let mut service =
    //         test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

    //     println!("{:#?}", user_1);
    //     let req = json!({
    //         "username": "muntasir",
    //         "name": "muntasir",
    //         "email": "muntasir@muntasir.com",
    //         "password": "muntasir",
    //     });

    //     println!("{:#?}", req);

    //     let resp = TestRequest::post()
    //         .uri("/user")
    //         .set_json(&req)
    //         .send_request(&mut service)
    //         .await;

    //     assert_eq!(resp.status(), StatusCode::CONFLICT , "Should throw user already exists error");
    // }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_user_get(_transaction: Transaction, user_1: User) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let resp = TestRequest::get()
            .uri(&format!("/user?id={}", user_1.id))
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to get user");

        let user: User = test::read_body_json(resp).await;
        assert_eq!(user_1.name, user.name);
        assert_eq!(user_1.username, user.username);
        assert_eq!(user_1.email, user.email);
        assert_eq!(user_1.password, user.password);
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_user_update(_transaction: Transaction, user_1: User) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let req = json!({
            "name": user_1.name.unwrap() + "updated",
            "username": user_1.username,
            "email": user_1.email,
            "password": user_1.password
        });

        let resp = TestRequest::put()
            .uri(&format!("/user?id={}", user_1.id))
            .set_json(&req)
            .send_request(&mut service)
            .await;

        assert_eq!(resp.status(), StatusCode::OK, "Failed to update user");
        let user: User = test::read_body_json(resp).await;
        assert_eq!(*req.get("name").unwrap(), user.name.unwrap());
        assert_eq!(*req.get("username").unwrap(), user.username);
        assert_eq!(*req.get("email").unwrap(), user.email);
        assert_eq!(*req.get("password").unwrap(), user.password.unwrap());
    }

    #[rstest]
    #[trace]
    #[actix_rt::test]
    async fn test_user_delete(_transaction: Transaction, user_1: User) {
        let mut service =
            test::init_service(ActixApp::new().configure(|cfg| api_routes::config(cfg, ""))).await;

        let resp = TestRequest::delete()
            .uri(&format!("/user?id={}", user_1.id))
            .send_request(&mut service)
            .await;

        assert_eq!(
            resp.status(),
            StatusCode::NO_CONTENT,
            "Failed to delete user"
        );
    }
}
