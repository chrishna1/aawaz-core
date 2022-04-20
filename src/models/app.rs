use crate::schema::app;

#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "app"]
pub struct App {
    pub id: i32,
    pub name: String,
    pub domain: String,
    pub owner: i32,
    pub deleted: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
