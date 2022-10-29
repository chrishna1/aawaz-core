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
mod controllers;
mod db;
mod models;
#[rustfmt::skip]
mod schema;
mod auth;
mod traits;
mod util;
