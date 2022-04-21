use crate::schema::page;

#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "page"]
pub struct Page {
    pub id: i32,
    pub app_id: i32,
    pub slug: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
