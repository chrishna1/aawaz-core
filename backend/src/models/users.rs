use crate::schema::users;
use crate::traits::CRUD;
use diesel::{dsl::*, pg::PgConnection, result::Error, *};

#[derive(
    Clone, Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Deserialize,
)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub ext_id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub name: Option<String>,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UserForm {
    pub username: String,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserParams {
    pub id: i32,
}

#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(Deserialize, PartialEq, Debug))]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        // TODO(improve) better way?
        Self {
            id: u.id,
            username: u.username,
            name: u.name,
            created_at: u.created_at,
        }
    }
}

impl CRUD for User {
    type IdType = i32;
    type Form = UserForm;

    fn create(conn: &PgConnection, form: &Self::Form) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        insert_into(users).values(form).get_result(conn)
    }

    fn read(conn: &PgConnection, user_id: i32) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        users.find(user_id).first::<Self>(conn)
    }

    fn update(conn: &PgConnection, user_id: i32, form: &Self::Form) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        update(users.find(user_id))
            .set(form)
            .get_result::<Self>(conn)
    }

    fn delete(conn: &PgConnection, user_id: i32) -> Result<usize, Error> {
        use crate::schema::users::dsl::*;
        delete(users.find(user_id)).execute(conn)
    }
}