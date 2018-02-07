use skrcime::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! post {
        it "responds with 201 created" {
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

    describe! get_by_email {
        it "responds with JSON body" {
            let res = client.get("/users/test@email.com").dispatch();

            assert_eq!(res.status(), Status::Ok);
        }
    }
}
