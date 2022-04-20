#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;

pub mod schema;
pub mod models;
pub mod controllers;
pub mod api_routes;
pub mod db;