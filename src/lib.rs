#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;

pub mod api_routes;
pub mod controllers;
pub mod db;
pub mod models;
pub mod schema;
