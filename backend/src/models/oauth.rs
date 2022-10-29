use crate::schema::oauth;
use diesel::*;

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "oauth"]
pub struct OauthForm {
    pub user_id: i32,
    pub provider_id: String,
    pub provider: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i32>,
}
