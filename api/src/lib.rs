#![deny(clippy::all)]

#[allow(unused_imports)]
#[macro_use]
extern crate log;

extern crate api_models as model;

pub mod backend;
pub mod controller;
pub mod data;
pub mod error;
