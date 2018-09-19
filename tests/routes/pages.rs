use rocket::http::Status;
use rocket::local::Client;
use skrcime::rocket;

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! login {
        it "responds with HTML" {
            let mut res = client.get("/prijava").dispatch();
            let body = res.body_string().unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert!(body.contains("Prijava"));
        }
  }
}
