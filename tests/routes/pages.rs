use skrcime::rocket;
use rocket::local::Client;
use rocket::http::Status;

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! login {
        it "responds with HTML" {
            let mut res = client.get("/login").dispatch();
            let body = res.body_string().unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert!(body.contains("Hello Skrci.me"));
        }
  }
}
