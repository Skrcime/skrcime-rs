use diesel::prelude::*;
use diesel::result::Error::NotFound;

use rocket::http::{Cookie, Cookies, Status};
use rocket::response::status::{Created, Custom};
use rocket::response::Redirect;
use rocket_contrib::{Json, Value};

use bcrypt::verify;
use validator::Validate;

use super::response::{error_message, error_validation};

use db::models::User;
use db::request::DbConnection;

pub static COOKIE_KEY: &'static str = "sk_s";

#[derive(Serialize, Deserialize, Validate)]
pub struct Credentials {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
}

#[derive(Debug, Copy, Clone)]
pub struct Session(pub i32);

#[post("/", format = "application/json", data = "<cred>")]
pub fn create(
    conn: DbConnection,
    cred: Json<Credentials>,
    mut cookies: Cookies,
) -> Result<Created<Json<Value>>, Custom<Json<Value>>> {
    if let Err(err) = cred.validate() {
        return Err(error_validation(err));
    }

    use db::schema::users::dsl;
    dsl::users
        .filter(dsl::email.eq(&cred.email.to_string()))
        .first::<User>(&*conn)
        .map_err(|err| match err {
            NotFound => error_message(Status::Unauthorized, "Invalid email or password"),
            _ => error_message(Status::InternalServerError, "Internal server error"),
        })
        .and_then(|user: User| {
            verify(&cred.password.to_string(), &user.password)
                .map_err(|_| error_message(Status::InternalServerError, "Internal server error"))
                .and_then(|valid| {
                    if valid {
                        let cookie = get_cookie(user.id.to_string());
                        cookies.add_private(cookie);

                        Ok(Created(
                            "/".to_string(),
                            Some(Json(json!({ "message": "Session created" }))),
                        ))
                    } else {
                        Err(error_message(
                            Status::Unauthorized,
                            "Invalid email or password",
                        ))
                    }
                })
        })
}

#[delete("/")]
pub fn destroy(mut cookies: Cookies) -> Redirect {
    let cookie = get_cookie("".to_string());
    cookies.remove_private(cookie);
    Redirect::to("/prijava")
}

fn get_cookie(value: String) -> Cookie<'static> {
    Cookie::build(COOKIE_KEY, value)
        .path("/")
        .http_only(true)
        .domain("skrci.me")
        // .secure(true)
        .finish()
}
