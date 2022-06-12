use crate::schema::page;
use crate::traits::CRUD;
use diesel::{dsl::*, pg::PgConnection, result::Error, *};

#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "page"]
pub struct Page {
    pub id: i32,
    pub ext_id: uuid::Uuid,
    pub app_id: i32,
    pub slug: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "page"]
pub struct PageForm {
    pub app_id: i32,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct PageParams {
    pub id: i32,
}

impl CRUD for Page {
    type IdType = i32;
    type Form = PageForm;

    fn create(conn: &PgConnection, form: &Self::Form) -> Result<Self, Error> {
        use crate::schema::page::dsl::*;
        insert_into(page).values(form).get_result(conn)
    }

    fn read(conn: &PgConnection, page_id: i32) -> Result<Self, Error> {
        use crate::schema::page::dsl::*;
        page.find(page_id).first::<Self>(conn)
    }

    fn update(conn: &PgConnection, page_id: i32, form: &Self::Form) -> Result<Self, Error> {
        use crate::schema::page::dsl::*;
        update(page.find(page_id))
            .set(form)
            .get_result::<Self>(conn)
    }

    fn delete(conn: &PgConnection, page_id: i32) -> Result<usize, Error> {
        use crate::schema::page::dsl::*;
        delete(page.find(page_id)).execute(conn)
    }
}

impl Page {
    pub fn list(conn: &PgConnection, app_id: i32) -> Result<Vec<Self>, Error> {
        use crate::schema::page::dsl;

        dsl::page.filter(dsl::app_id.eq(app_id)).load::<Page>(conn)
    }
}