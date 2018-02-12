extern crate rocket;
extern crate serde_json;

use std::io::Read;
use rocket::response::Body;
use serde_json::{from_str, Value};

pub fn json_body(body: Option<Body<&mut Read>>) -> Value {
    body.and_then(|b| from_str(&b.into_string().unwrap()).unwrap())
        .unwrap()
}
