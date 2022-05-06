use rstest::fixture;

use crate::db::get_db_connection;
use crate::models::{App, AppForm, User};
use crate::test::fixtures::user_1;
use crate::traits::CRUD;

#[fixture]
pub fn app_1(user_1: User) -> App {
    let conn = get_db_connection();

    let app_form = AppForm {
        name: String::from("muntasir blog\"s comment"),
        domain: String::from("https://muntasir.com"),
        owner: user_1.id,
    };

    App::create(&conn, &app_form).unwrap()
}
