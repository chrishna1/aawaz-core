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
        password: Some(String::from("muntasir")),
    };
    User::create(&conn, &user_form).unwrap()
}

#[fixture]
pub fn user_2() -> User {
    let conn = get_db_connection();
    let user_form = UserForm {
        username: String::from("muntasir_2"),
        name: Some(String::from("munatasir_2")),
        email: String::from("muntasir_2@muntasir_2.com"),
        password: Some(String::from("muntasir_2")),
    };
    User::create(&conn, &user_form).unwrap()
}
