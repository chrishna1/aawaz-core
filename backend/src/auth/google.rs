use crate::models::{OAuthStates, OAuthStatesForm, StateData, UserExtra, UserExtraForm};
use crate::traits::CRUD;
use crate::{db, models::UserForm};
use actix_session::Session;
use actix_web::http::{header, Method};
use actix_web::{web, HttpResponse};
use diesel::insert_into;
use dotenv::dotenv;
use oauth2::{
    basic::BasicClient, http::HeaderMap, reqwest::async_http_client, AccessToken, AuthUrl,
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse,
    TokenUrl,
};
use std::env;
use url::Url;

use super::util::{OauthPayload, COMMENT_FRAGMENT};
use crate::{models::OauthForm, models::User, util::EndpointResult};

#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    id: String,
    name: String,       // full name
    given_name: String, // first name
    email: String,
}

async fn read_user(access_token: &AccessToken) -> UserInfo {
    let url = Url::parse(
        format!(
            "https://www.googleapis.com/oauth2/v2/userinfo?access_token={}",
            access_token.secret()
        )
        .as_str(),
    )
    .unwrap();

    println!("{}", url);

    let header = HeaderMap::new();

    let resp = async_http_client(oauth2::HttpRequest {
        url,
        method: Method::GET,
        headers: header,
        body: Vec::new(),
    })
    .await
    .expect("Request failed");

    let resultam: UserInfo = serde_json::from_slice(&resp.body).unwrap();

    println!("{:?}", resultam);

    resultam
}

pub async fn login(params: web::Query<OauthPayload>) -> EndpointResult {
    dotenv().ok();

    let client_id = ClientId::new(
        env::var("OAUTH_GOOGLE_CLIENT_ID").expect("OAUTH_GOOGLE_CLIENT_ID must be set"),
    );
    let client_secret = ClientSecret::new(
        env::var("OAUTH_GOOGLE_SECRET").expect("OAUTH_GOOGLE_SECRET must be set"),
    );

    let auth_url = AuthUrl::new(String::from("https://accounts.google.com/o/oauth2/auth")).unwrap();

    let token_url = TokenUrl::new(String::from("https://oauth2.googleapis.com/token")).unwrap();

    let redirect_url = RedirectUrl::new(
        env::var("BASE_URL").expect("BASE_URL must be set") + "/auth/google/callback",
    )
    .unwrap();

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    let state_token = CsrfToken::new_random();

    let connection = db::get_db_connection();

    let form = OAuthStatesForm {
        state_id: state_token.clone().secret().to_string(),
        state_data: serde_json::to_value(StateData {
            url: params.url.clone(),
        })
        .unwrap(),
    };

    OAuthStates::create(&connection, &form).unwrap();

    let (auth_url, _csrf_token) = client
        .authorize_url(|| state_token)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        // .set_pkce_challenge(pkce_code_challenge)
        .url();

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish())
}

pub async fn callback(session: Session, params: web::Query<AuthRequest>) -> EndpointResult {
    dotenv().ok();

    let client_id = ClientId::new(
        env::var("OAUTH_GOOGLE_CLIENT_ID").expect("OAUTH_GOOGLE_CLIENT_ID must be set"),
    );
    let client_secret = ClientSecret::new(
        env::var("OAUTH_GOOGLE_SECRET").expect("OAUTH_GOOGLE_SECRET must be set"),
    );

    let auth_url = AuthUrl::new(String::from("https://accounts.google.com/o/oauth2/auth")).unwrap();

    let token_url = TokenUrl::new(String::from("https://oauth2.googleapis.com/token")).unwrap();

    let redirect_url = RedirectUrl::new(
        env::var("BASE_URL").expect("BASE_URL must be set") + "/auth/google/callback",
    )
    .unwrap();

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());

    let connection = db::get_db_connection();
    let store = OAuthStates::from_state_id(&connection, state.secret()).unwrap();
    let state_data: StateData = serde_json::from_value(store.state_data).unwrap();

    let token = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .unwrap();

    println!("{}", token.access_token().secret());

    let user_response = read_user(token.access_token()).await;

    // check if email already exists
    // if not
    // create user
    let existing_user = User::from_email(&connection, &user_response.email);

    let uid;

    if existing_user.is_err() {
        // email not found!! create new user
        let form = UserForm {
            username: user_response.given_name,
            email: user_response.email,
            name: Some(user_response.name),
            password: None,
        };

        // TODO!!! - make following two insert atomic!!
        let new_user = match User::create(&connection, &form) {
            Ok(user) => user,
            Err(_) => return Ok(HttpResponse::InternalServerError().json("User creation failed")),
        };

        uid = new_user.id;

        let form = OauthForm {
            user_id: new_user.id,
            provider_id: user_response.id.to_string(),
            provider: String::from("google"),
            access_token: token.access_token().secret().to_string(),
            refresh_token: match token.refresh_token() {
                Some(token) => Some(token.secret().to_string()),
                None => None,
            },
            expires_at: match token.expires_in() {
                Some(duration) => Some(duration.as_secs().try_into().unwrap()),
                None => None,
            },
        };

        use crate::diesel::RunQueryDsl;
        use crate::schema::oauth::dsl::*;
        insert_into(oauth)
            .values(form)
            .execute(&connection)
            .unwrap();
    } else {
        uid = existing_user.unwrap().id;
    }

    let form = UserExtraForm {
        user_id: uid,
        source: Some(state_data.url.clone()),
    };

    UserExtra::create(&connection, form).unwrap();

    // set session cookie
    session
        .insert("uid", uid)
        .expect("Error in storing session");

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, state_data.url + COMMENT_FRAGMENT))
        .finish())
}
