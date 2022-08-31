#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;

#[cfg(test)]
mod test;

pub mod api_routes;
pub mod controllers;
pub mod db;
pub mod models;
#[rustfmt::skip]
pub mod schema;
pub mod traits;
pub mod util;
