use skrcime::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! create {
        it "should responds with 201 Created" {
            let res = client.post("/api/session")
                .body(&json!({
                    "email": "user@one.com",
                    "password": "secretone"
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();
            let set_cookie = res.headers().get_one("Set-Cookie").unwrap();

            assert_eq!(res.status(), Status::Created);
            assert!(set_cookie.contains("sk_s="));
            assert!(set_cookie.contains("HttpOnly"));
        }

        it "should responds with 401 if no user exists with that email" {
            let res = client.post("/api/session")
                .body(&json!({
                    "email": "user@foo.com",
                    "password": "secretone"
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::Unauthorized);
        }

        it "should responds with 401 if password wrong" {
            let res = client.post("/api/session")
                .body(&json!({
                    "email": "user@one.com",
                    "password": "secretone-foo"
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::Unauthorized);
        }

        it "should responds with 400 if email invalid" {
            let res = client.post("/api/session")
                .body(&json!({
                    "email": "foooo",
                    "password": "secretone"
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::BadRequest);
        }

        it "should responds with 400 if password invalid" {
            let res = client.post("/api/session")
                .body(&json!({
                    "email": "user@one.com",
                    "password": ""
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::BadRequest);
        }
    }
}
