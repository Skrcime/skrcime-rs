use diesel::prelude::*;

use rocket_contrib::{Json, Value};
use rocket::http::{Cookie, Cookies};

use bcrypt::verify;

use db::request::DbConnection;
use db::models::Users;
use db::schema::users::dsl::*;

static COOKIE_KEY: &'static str = "sk_s";

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[post("/", format = "application/json", data = "<cred>")]
pub fn create(conn: DbConnection, cred: Json<Credentials>, mut cookies: Cookies) -> Json<Value> {
    let result = users
        .filter(email.eq(&cred.email.to_string()))
        .first::<Users>(&*conn);

    match result {
        Ok(user) => match verify(&cred.password.to_string(), &user.password) {
            Ok(valid) => {
                if valid {
                    let user_id = format!("{}", &user.id.to_string());
                    let mut cookie = Cookie::new(COOKIE_KEY, user_id);
                    cookie.set_http_only(true);
                    // cookie.set_secure(true);
                    cookies.add_private(cookie);

                    Json(json!({ "success": "ok" }))
                } else {
                    Json(json!({ "error": "Invalid email or password" }))
                }
            }
            Err(err) => Json(json!({"error": err.to_string()})),
        },
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}
