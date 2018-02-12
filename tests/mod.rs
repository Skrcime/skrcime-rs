#![feature(plugin)]
#![plugin(stainless)]

extern crate rand;
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate skrcime;

mod utils;

#[cfg(test)]
mod routes;
