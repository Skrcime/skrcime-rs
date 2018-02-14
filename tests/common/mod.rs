extern crate rocket;

use rocket::Response;
use rocket::local::Client;
use rocket::http::{Cookie, ContentType};

use serde_json::{from_str, Value};

pub fn json_body(body: Option<String>) -> Option<Value> {
    from_str(&body.unwrap()).unwrap()
}

pub fn user_id_cookie(response: &Response) -> Option<Cookie<'static>> {
    let cookie = response.headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with("sk_s"))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}

pub fn login_cookie(client: &Client, email: &str, pass: &str) -> Option<Cookie<'static>> {
    let response = client.post("/api/session")
        .body(&json!({
            "email": email,
            "password": pass
        }).to_string())
        .header(ContentType::JSON)
        .dispatch();

    user_id_cookie(&response)
}
