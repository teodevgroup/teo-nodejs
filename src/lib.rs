#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod handler;
pub mod middleware;
pub mod model;
pub mod namespace;
pub mod object;
pub mod result;
pub mod r#struct;
pub mod app;