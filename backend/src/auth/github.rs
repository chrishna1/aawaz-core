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

use crate::{models::OauthForm, models::User, util::EndpointResult};

use super::util::{OauthPayload, COMMENT_FRAGMENT};

#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    id: u64,
    name: String,
    login: String,
    email: String,
}

#[derive(Deserialize, Debug)]
pub struct GhEmail {
    email: String,
    primary: bool,
    // verified: bool TODO - send verification email for unverified.
}

async fn get_primary_email(access_token: &AccessToken) -> Option<String> {
    let url = Url::parse("https://api.github.com/user/emails").unwrap();

    println!("{}", url);

    let mut header = HeaderMap::new();
    header.append(
        header::AUTHORIZATION,
        format!("token {}", access_token.secret())
            .as_str()
            .parse()
            .unwrap(), // wtf??
    );

    header.append(
        header::ACCEPT,
        "application/vnd.github.v3+json".parse().unwrap(),
    );
    header.append(header::USER_AGENT, "aawaz.io".parse().unwrap());

    let resp = async_http_client(oauth2::HttpRequest {
        url,
        method: Method::GET,
        headers: header,
        body: Vec::new(),
    })
    .await
    .expect("Request failed");

    // let s = String::from_utf8(resp.body).expect("Found invalid UTF-8");
    // println!("{}", s);
    // println!("{:?}", resp.body);

    let resultam: Vec<GhEmail> = serde_json::from_slice(&resp.body).unwrap();

    println!("{:?}", resultam);

    for record in resultam {
        if record.primary {
            return Some(record.email);
        }
    }

    return None;
}

async fn read_user(access_token: &AccessToken) -> UserInfo {
    let url = Url::parse("https://api.github.com/user").unwrap();

    println!("{}", url);

    let mut header = HeaderMap::new();
    header.append(
        header::AUTHORIZATION,
        format!("token {}", access_token.secret())
            .as_str()
            .parse()
            .unwrap(), // wtf??
    );

    header.append(
        header::ACCEPT,
        "application/vnd.github.v3+json".parse().unwrap(),
    );
    header.append(header::USER_AGENT, "aawaz.io".parse().unwrap());

    let resp = async_http_client(oauth2::HttpRequest {
        url,
        method: Method::GET,
        headers: header,
        body: Vec::new(),
    })
    .await
    .expect("Request failed");

    // let s = String::from_utf8(resp.body).expect("Found invalid UTF-8");
    // println!("{}", s);
    // println!("{:?}", resp.body);

    let resultam: UserInfo = serde_json::from_slice(&resp.body).unwrap();

    println!("{:?}", resultam);

    resultam
}

pub async fn login(params: web::Query<OauthPayload>) -> EndpointResult {
    dotenv().ok();

    let client_id = ClientId::new(
        env::var("OAUTH_GITHUB_CLIENT_ID").expect("OAUTH_GITHUB_CLIENT_ID must be set"),
    );
    let client_secret = ClientSecret::new(
        env::var("OAUTH_GITHUB_SECRET").expect("OAUTH_GITHUB_SECRET must be set"),
    );

    let auth_url = AuthUrl::new(String::from("https://github.com/login/oauth/authorize")).unwrap();

    let token_url =
        TokenUrl::new(String::from("https://github.com/login/oauth/access_token")).unwrap();

    let redirect_url = RedirectUrl::new(
        env::var("BASE_URL").expect("BASE_URL must be set") + "/auth/github/callback",
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
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish())
}

pub async fn callback(session: Session, params: web::Query<AuthRequest>) -> EndpointResult {
    dotenv().ok();

    let client_id = ClientId::new(
        env::var("OAUTH_GITHUB_CLIENT_ID").expect("OAUTH_GITHUB_CLIENT_ID must be set"),
    );
    let client_secret = ClientSecret::new(
        env::var("OAUTH_GITHUB_SECRET").expect("OAUTH_GITHUB_SECRET must be set"),
    );

    let auth_url = AuthUrl::new(String::from("https://github.com/login/oauth/authorize")).unwrap();

    let token_url =
        TokenUrl::new(String::from("https://github.com/login/oauth/access_token")).unwrap();

    let redirect_url = RedirectUrl::new(
        env::var("BASE_URL").expect("BASE_URL must be set") + "/auth/github/callback",
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

    let email = get_primary_email(token.access_token())
        .await
        .unwrap_or_else(|| user_response.email);

    // check if email already exists
    // if not
    // create user
    let existing_user = User::from_email(&connection, &email);

    let uid;

    if existing_user.is_err() {
        // email not found!! create new user
        let form = UserForm {
            username: user_response.login,
            email: email,
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
            provider: String::from("github"),
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
