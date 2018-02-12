use diesel::prelude::*;
use diesel::result::Error::NotFound;

use rocket_contrib::Json;
use rocket::response::{status, Failure};
use rocket::http::{Cookie, Cookies, Status};

use bcrypt::verify;

use db::request::DbConnection;
use db::models::User;

pub static COOKIE_KEY: &'static str = "sk_s";

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Session(pub i32);

#[post("/", format = "application/json", data = "<cred>")]
pub fn create(
    conn: DbConnection,
    cred: Json<Credentials>,
    mut cookies: Cookies,
) -> Result<status::Created<String>, Failure> {
    use db::schema::users::dsl;

    dsl::users
        .filter(dsl::email.eq(&cred.email.to_string()))
        .first::<User>(&*conn)
        .map_err(|err| match err {
            NotFound => Failure(Status::Unauthorized),
            _ => Failure(Status::InternalServerError),
        })
        .and_then(|user: User| {
            verify(&cred.password.to_string(), &user.password)
                .map_err(|_| Failure(Status::InternalServerError))
                .and_then(|valid| {
                    if valid {
                        let mut cookie = Cookie::new(COOKIE_KEY, user.id.to_string());
                        cookie.set_http_only(true);
                        // cookie.set_secure(true);
                        cookies.add_private(cookie);

                        Ok(status::Created("".to_string(), Some("Success".to_string())))
                    } else {
                        Err(Failure(Status::Unauthorized))
                    }
                })
        })
}
