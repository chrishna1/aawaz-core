use crate::schema::{oauth, oauth_states};
use diesel::{dsl::*, pg::PgConnection, result::Error, *};

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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StateData {
    pub url: String,
}

#[derive(
    Clone, Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Deserialize,
)]
#[table_name = "oauth_states"]
pub struct OAuthStates {
    pub id: i32,
    pub state_id: String,
    pub state_data: serde_json::Value,
}

#[derive(Deserialize, Insertable)]
#[table_name = "oauth_states"]
pub struct OAuthStatesForm {
    pub state_id: String,
    pub state_data: serde_json::Value,
}

impl OAuthStates {
    pub fn create(conn: &PgConnection, form: &OAuthStatesForm) -> Result<Self, Error> {
        use crate::schema::oauth_states::dsl::*;
        insert_into(oauth_states).values(form).get_result(conn)
    }

    pub fn from_state_id(conn: &PgConnection, state_id: &str) -> Result<Self, Error> {
        use crate::schema::oauth_states::dsl;
        dsl::oauth_states
            .filter(dsl::state_id.eq(state_id))
            .first::<Self>(conn)
    }
}
