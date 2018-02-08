use skrcime::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};
use serde_json::{from_str, Value};

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
        let new_user = &json!({
            "first_name": "Foo",
            "last_name": "Bar",
            "avatar_url": "https://avatar.png",
            "email": "test@email.com",
            "password": "test"
        });
    }

    describe! create {
        it "should responds with 201 Created" {
            let res = client.post("/users")
                .body(new_user.to_string())
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::Created);
            assert_eq!(res.headers().get_one("Location").unwrap(), "/users/1");
        }
    }

    describe! get {
        it "responds with 200 Ok" {
            let mut res = client.get("/users/1").dispatch();
            let body: Value = res.body().and_then(|b| {
                from_str(&b.into_string().unwrap()).unwrap()
            }).unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["id"], 1);
            assert_eq!(body["first_name"], new_user["first_name"]);
            assert_eq!(body["last_name"], new_user["last_name"]);
            assert_eq!(body["avatar_url"], new_user["avatar_url"]);
            assert_eq!(body["email"], new_user["email"]);
            assert!(body["password"].is_string());
            assert_eq!(body["admin"], false);
            assert_eq!(body["welcome"], true);
            assert!(body["created_at"].is_string());
        }
    }
}
