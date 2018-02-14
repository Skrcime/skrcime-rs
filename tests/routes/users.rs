use skrcime::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};
use serde_json::Value;
use common::{json_body, login_cookie};

describe! test {
    before_each {
        let client = Client::new(rocket()).unwrap();
    }

    describe! create {
        it "should responds with 201 and user body" {
            let mut res = client.post("/api/users")
                .body(&json!({
                    "name": "User",
                    "email": "user@one.com",
                    "password": "secretone"
                }).to_string())
                .header(ContentType::JSON)
                .dispatch();
            let body = json_body(res.body_string()).unwrap();

            assert_eq!(res.status(), Status::Created);
            assert_eq!(res.headers().get_one("Location").unwrap(), "/users/1");

            assert_eq!(body["id"], 1);
            assert_eq!(body["name"], "User");
            assert_eq!(body["avatar_url"], Value::Null);
            assert_eq!(body["email"], "user@one.com");
            assert_eq!(body.get("password"), None);
            assert_eq!(body["admin"], false);
            assert_eq!(body["welcome"], true);
            assert!(body["created_at"].is_string());
        }

        it "should responds with 201 with avatar_url" {
            let res = client.post("/api/users")
                .body(&json!({
                    "name": "User",
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
            let res = client.post("/api/users")
                .body(&json!({
                    "name": "User",
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
            let cookie = login_cookie(&client, "user@one.com", "secretone").unwrap();
            let mut res = client.get("/api/users/me").cookie(cookie.clone()).dispatch();

            let body = json_body(res.body_string()).unwrap();
            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["id"], 1);
            assert_eq!(body["name"], "User");
            assert_eq!(body["avatar_url"], Value::Null);
            assert_eq!(body["email"], "user@one.com");
            assert_eq!(body.get("password"), None);
            assert_eq!(body["admin"], false);
            assert_eq!(body["welcome"], true);
            assert!(body["created_at"].is_string());
        }

        it "should respond with 401 if not logged in" {
            let res = client.get("/api/users/me").dispatch();

            assert_eq!(res.status(), Status::Unauthorized);
        }
    }

    describe! update {
        before_each {
            let _cookie = login_cookie(&client, "user@two.com", "secrettwo").unwrap();
        }

        it "should update name" {
            let mut res = client.patch("/api/users/me")
                .cookie(_cookie.clone())
                .header(ContentType::JSON)
                .body(&json!({
                    "name": "User Updated",
                }).to_string())
                .dispatch();
            let body = json_body(res.body_string()).unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["name"], "User Updated");
        }

        it "should update avatar_url" {
            let mut res = client.patch("/api/users/me")
                .cookie(_cookie.clone())
                .header(ContentType::JSON)
                .body(&json!({
                    "avatar_url": "https://avatar-updated.png",
                }).to_string())
                .dispatch();
            let body = json_body(res.body_string()).unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["avatar_url"], "https://avatar-updated.png");
        }

        it "should update welcome" {
            let mut res = client.patch("/api/users/me")
                .cookie(_cookie.clone())
                .header(ContentType::JSON)
                .body(&json!({
                    "welcome": false,
                }).to_string())
                .dispatch();
            let body = json_body(res.body_string()).unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["welcome"], false);
        }


        it "should respond with 401 if not logged in" {
            let res = client.patch("/api/users/me")
                .header(ContentType::JSON)
                .body(&json!({
                    "name": "User Updated",
                }).to_string())
                .dispatch();

            assert_eq!(res.status(), Status::Unauthorized);
        }
    }

    describe! update_email {
        it "should update email" {
            let cookie = login_cookie(&client, "user@two.com", "secrettwo").unwrap();
            let mut res = client.patch("/api/users/me")
                .cookie(cookie.clone())
                .header(ContentType::JSON)
                .body(&json!({
                    "email": "user@two-updated.com",
                }).to_string())
                .dispatch();
            let body = json_body(res.body_string()).unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["email"], "user@two-updated.com");
        }
    }

    describe! update_password {
        it "should update password" {
            let cookie = login_cookie(&client, "user@two-updated.com", "secrettwo").unwrap();
            let res = client.patch("/api/users/me")
                .cookie(cookie.clone())
                .header(ContentType::JSON)
                .body(&json!({
                    "password": "supersecret-new",
                }).to_string())
                .dispatch();

            assert_eq!(res.status(), Status::Ok);
        }
    }
}
