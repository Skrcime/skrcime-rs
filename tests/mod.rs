#![feature(plugin)]
#![plugin(stainless)]

extern crate rand;
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate skrcime;

#[cfg(test)]
mod routes;
