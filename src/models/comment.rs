use crate::schema::comment;
use crate::traits::CRUD;

use diesel::{dsl::*, pg::PgConnection, result::Error, *};

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

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "comment"]
pub struct CommentForm {
    pub user_id: i32,
    pub page_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
}

// TODO - since crud impls are similar everywhere, can it be derived.
impl CRUD for Comment {
    type IdType = i32;
    type Form = CommentForm;

    fn create(conn: &PgConnection, form: &Self::Form) -> Result<Self, Error> {
        use crate::schema::comment::dsl::*;
        insert_into(comment).values(form).get_result(conn)
    }

    fn read(conn: &PgConnection, comment_id: i32) -> Result<Self, Error> {
        use crate::schema::comment::dsl::*;
        comment.find(comment_id).first::<Self>(conn)
    }

    fn update(conn: &PgConnection, app_id: i32, form: &Self::Form) -> Result<Self, Error> {
        use crate::schema::comment::dsl::*;
        update(comment.find(app_id))
            .set(form)
            .get_result::<Self>(conn)
    }

    fn delete(conn: &PgConnection, app_id: i32) -> Result<usize, Error> {
        use crate::schema::comment::dsl::*;
        delete(comment.find(app_id)).execute(conn)
    }
}

impl Comment {
    pub fn list(conn: &PgConnection, page_id: i32) -> Result<Vec<Self>, Error> {
        use crate::schema::comment::dsl;

        dsl::comment
            .filter(dsl::page_id.eq(page_id))
            .load::<Comment>(conn)
    }
}
