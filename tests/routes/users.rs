use skrcime::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! create {
        it "should responds with 201 Created" {
            let payload = &json!({
                "first_name": "Foo",
                "last_name": "Bar",
                "email": "test@email.com",
                "password": "test"
            }).to_string();

            let res = client.post("/users")
                .body(payload)
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::Created);
        }
    }

    describe! get {
        it "responds with 200 Ok" {
            let res = client.get("/users/39").dispatch();

            assert_eq!(res.status(), Status::Ok);
        }
    }
}
