use crate::schema::comment;

#[derive(
  Clone, Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Deserialize,
)]
#[table_name = "comment"]
pub struct Comment {
  pub id: i32,
  pub created_by: i32,
  pub page_id: i32,
  pub parent_id: Option<i32>,
  pub content: String,
  pub deleted: bool,
  pub updated_at: Option<chrono::NaiveDateTime>,
  pub created_at: Option<chrono::NaiveDateTime>,
}