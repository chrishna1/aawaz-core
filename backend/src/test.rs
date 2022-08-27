use crate::db;
use diesel::{connection::TransactionManager, Connection};
use rstest::fixture;

pub mod fixtures;

#[cfg(test)]
pub fn init() {
    db::init();
}

#[derive(Debug)]
pub struct Transaction {}

#[fixture]
pub fn transaction() -> Transaction {
    crate::test::init();
    Transaction {}
}

impl Drop for Transaction {
    fn drop(&mut self) {
        let conn = crate::db::get_db_connection();

        let transaction_manager = conn.transaction_manager();
        transaction_manager.rollback_transaction(&conn).unwrap();
    }
}
