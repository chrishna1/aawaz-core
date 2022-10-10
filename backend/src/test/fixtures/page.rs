use rstest::fixture;

use crate::db::get_db_connection;
use crate::models::{App, Page, PageForm};
use crate::test::fixtures::app_1;
use crate::traits::CRUD;

#[fixture]
pub fn page_1(app_1: App) -> Page {
    let conn = get_db_connection();
    let page_form = PageForm {
        app_id: app_1.id,
        path: String::from("/zindagi-kaisi-hai-paheli"),
    };
    Page::create(&conn, &page_form).unwrap()
}

#[fixture]
pub fn page_2(app_1: App) -> Page {
    let conn = get_db_connection();
    let page_form = PageForm {
        app_id: app_1.id,
        path: String::from("/zindagi-ka-safar"),
    };
    Page::create(&conn, &page_form).unwrap()
}

#[fixture]
pub fn page_with_threads(app_1: App) -> Page {
    let conn = get_db_connection();
    let page_form = PageForm {
        app_id: app_1.id,
        path: String::from("/ajeeb-dastan-hai-ye"),
    };
    Page::create(&conn, &page_form).unwrap()
}
