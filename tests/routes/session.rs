use skrcime::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! create {
        it "should responds with 201 Created" {
            let login = &json!({
                "email": "test@email.com",
                "password": "test"
            });
            let res = client.post("/session")
                .body(login.to_string())
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::Created);
        }
    }
}
