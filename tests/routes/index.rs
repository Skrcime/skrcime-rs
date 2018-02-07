use skrcime::rocket;
use rocket::local::Client;
use rocket::http::Status;

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    it "responds with HTTP 200 and HTML body" {
        let mut res = client.get("/").dispatch();
        let body = res.body().and_then(|b| b.into_string()).unwrap();

        assert_eq!(res.status(), Status::Ok);
        assert!(body.contains("Hello Skrci.me"));
    }
}
