use crate::schema::comment;

#[derive(
    Clone, Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Deserialize,
)]
#[table_name = "comment"]
pub struct Comment {
    pub id: i32,
    pub ext_id: uuid::Uuid,
    pub user_id: i32,
    pub page_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
