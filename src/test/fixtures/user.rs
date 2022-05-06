use rstest::fixture;

use crate::db::get_db_connection;
use crate::models::{User, UserForm};
use crate::traits::CRUD;

#[fixture]
pub fn user_1() -> User {
    let conn = get_db_connection();
    let user_form = UserForm {
        username: String::from("muntasir"),
        name: Some(String::from("munatasir")),
        email: String::from("muntasir@muntasir.com"),
        password: String::from("muntasir"),
    };
    User::create(&conn, &user_form).unwrap()
}
