use crate::schema::app;
use crate::traits::CRUD;
use diesel::{dsl::*, pg::PgConnection, result::Error, *};

#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "app"]
pub struct App {
    pub id: i32,
    pub ext_id: uuid::Uuid,
    pub name: String,
    pub domain: String,
    pub owner: i32,
    pub is_deleted: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "app"]
pub struct AppForm {
    pub name: String,
    pub domain: String,
    pub owner: i32,
}

impl CRUD for App {
    type IdType = i32;
    type Form = AppForm;

    fn create(conn: &PgConnection, form: &Self::Form) -> Result<Self, Error> {
        use crate::schema::app::dsl::*;
        insert_into(app).values(form).get_result(conn)
    }

    fn read(conn: &PgConnection, app_id: i32) -> Result<Self, Error> {
        use crate::schema::app::dsl::*;
        app.find(app_id).first::<Self>(conn)
    }

    fn update(conn: &PgConnection, app_id: i32, form: &Self::Form) -> Result<Self, Error> {
        use crate::schema::app::dsl::*;
        update(app.find(app_id)).set(form).get_result::<Self>(conn)
    }

    fn delete(conn: &PgConnection, app_id: i32) -> Result<usize, Error> {
        use crate::schema::app::dsl::*;
        delete(app.find(app_id)).execute(conn)
    }
}
