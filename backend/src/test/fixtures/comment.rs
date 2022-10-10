use rstest::fixture;

use crate::db::get_db_connection;
use crate::models::{Comment, CommentForm, Page, User};
use crate::test::fixtures::{page_1, page_with_threads, user_2};
use crate::traits::CRUD;

#[fixture]
pub fn comment_1(user_2: User, page_1: Page) -> Comment {
    let conn = get_db_connection();

    let form = CommentForm {
        user_id: user_2.id,
        page_id: page_1.id,
        parent_id: None,
        content: String::from("behtareen"),
    };

    Comment::create(&conn, &form).unwrap()
}

#[fixture]
pub fn comment_with_threads_1(user_2: User, page_with_threads: Page) -> (Comment, Comment) {
    let conn = get_db_connection();
    let form = CommentForm {
        user_id: user_2.id,
        page_id: page_with_threads.id,
        parent_id: None,
        content: String::from("behtareen"),
    };

    let root_comment = Comment::create(&conn, &form).unwrap();

    let child_comment_form = CommentForm {
        user_id: user_2.id,
        page_id: page_with_threads.id,
        parent_id: Some(root_comment.id),
        content: String::from("behtareen k bachha"),
    };

    let child_comment = Comment::create(&conn, &child_comment_form).unwrap();

    (root_comment, child_comment)
}
