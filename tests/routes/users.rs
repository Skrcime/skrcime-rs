use skrcime::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};
use serde_json::Value;
use utils::json_body;

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! create {
        it "should responds with 201 and user body" {
            let mut res = client.post("/users")
                .body(&json!({
                    "first_name": "User",
                    "last_name": "One",
                    "email": "user@one.com",
                    "password": "secretone"
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();
            let body = json_body(res.body());

            assert_eq!(res.status(), Status::Created);
            assert_eq!(res.headers().get_one("Location").unwrap(), "/users/1");

            assert_eq!(body["id"], 1);
            assert_eq!(body["first_name"], "User");
            assert_eq!(body["last_name"], "One");
            assert_eq!(body["avatar_url"], Value::Null);
            assert_eq!(body["email"], "user@one.com");
            assert_eq!(body.get("password"), None);
            assert_eq!(body["admin"], false);
            assert_eq!(body["welcome"], true);
            assert!(body["created_at"].is_string());
        }

        it "should responds with 201 with avatar_url" {
            let res = client.post("/users")
                .body(&json!({
                    "first_name": "User",
                    "last_name": "Two",
                    "avatar_url": "https://avatar.png",
                    "email": "user@two.com",
                    "password": "secrettwo"
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::Created);
            assert_eq!(res.headers().get_one("Location").unwrap(), "/users/2");
        }

        it "should return 409 if user with the same email exists" {
            let res = client.post("/users")
                .body(&json!({
                    "first_name": "User",
                    "last_name": "Two",
                    "email": "user@two.com",
                    "password": "secrettwo"
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::Conflict);
        }
    }

    describe! get {
        it "should responds with 200 and user body" {
            let mut res = client.get("/users/1").dispatch();
            let body = json_body(res.body());

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["id"], 1);
            assert_eq!(body["first_name"], "User");
            assert_eq!(body["last_name"], "One");
            assert_eq!(body["avatar_url"], Value::Null);
            assert_eq!(body["email"], "user@one.com");
            assert_eq!(body.get("password"), None);
            assert_eq!(body["admin"], false);
            assert_eq!(body["welcome"], true);
            assert!(body["created_at"].is_string());
        }

        it "should respond with 404 if no user found" {
            let res = client.get("/users/999").dispatch();

            assert_eq!(res.status(), Status::NotFound);
        }
    }

    describe! update {
        it "should update first_name" {
            let mut res = client.patch("/users/2")
                .body(&json!({
                    "first_name": "User Updated",
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();
            let body = json_body(res.body());

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["first_name"], "User Updated");
        }

        it "should update last_name" {
            let mut res = client.patch("/users/2")
                .body(&json!({
                    "last_name": "Two Updated",
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();
            let body = json_body(res.body());

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["last_name"], "Two Updated");
        }

        it "should update email" {
            let mut res = client.patch("/users/2")
                .body(&json!({
                    "email": "email@two-updated.com",
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();
            let body = json_body(res.body());

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["email"], "email@two-updated.com");
        }

        it "should update avatar_url" {
            let mut res = client.patch("/users/2")
                .body(&json!({
                    "avatar_url": "https://avatar-updated.png",
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();
            let body = json_body(res.body());

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["avatar_url"], "https://avatar-updated.png");
        }

        it "should update welcome" {
            let mut res = client.patch("/users/2")
                .body(&json!({
                    "welcome": false,
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();
            let body = json_body(res.body());

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["welcome"], false);
        }
    }
}
