use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

pub struct Subdomain(pub String);

// Expecting moj.skrci.me
fn is_valid(host: &str) -> Option<&str> {
    let vec: Vec<&str> = host.split('.').collect();
    if vec.len() == 3 && vec[0] == "moj" {
        return Some(vec[0]);
    }
    None
}

impl<'a, 'r> FromRequest<'a, 'r> for Subdomain {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Subdomain, ()> {
        let hosts: Vec<_> = request.headers().get("host").collect();
        match is_valid(hosts[0]) {
            Some(subdomain) => Outcome::Success(Subdomain(subdomain.to_string())),
            None => Outcome::Forward(()),
        }
    }
}
