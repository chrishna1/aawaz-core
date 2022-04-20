use crate::schema::page;

#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "page"]
pub struct Page {
    pub id: i32,
    pub app_id: i32,
    pub slug: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
