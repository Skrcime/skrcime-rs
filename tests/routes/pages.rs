use skrcime::rocket;
use rocket::local::Client;
use rocket::http::Status;

describe! pages {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! index {
        it "responds with HTML" {
            let mut res = client.get("/").dispatch();
            let body = res.body().and_then(|b| b.into_string()).unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert!(body.contains("Hello Skrci.me"));
        }
  }
}
