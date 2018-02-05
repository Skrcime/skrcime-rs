#![feature(plugin)]
#![plugin(stainless)]

extern crate rocket;
extern crate skrcime;

#[cfg(test)]
#[allow(unused_variables)]
mod routes {
    use super::skrcime::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    describe! index {
        before_each {
          let client = Client::new(rocket()).expect("valid rocket instance");
          let mut res = client.get("/").dispatch();
          let body = res.body().and_then(|b| b.into_string()).unwrap();
        }

        it "responds with status OK 200" {
            assert_eq!(res.status(), Status::Ok);
        }

        it "responds with HTML body" {
            assert!(body.contains("Hello Skrci.me"));
        }
    }
}
