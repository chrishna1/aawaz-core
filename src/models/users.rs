use crate::schema::users;

#[derive(
    Clone, Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Deserialize,
)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub handle: String,
    pub name: String,
    pub password: String,
    pub email: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
