use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error::{DatabaseError, NotFound};
use diesel::result::DatabaseErrorKind::UniqueViolation;

use rocket::response::{status, Failure};
use rocket::http::Status;
use rocket_contrib::{Json, Value};

use bcrypt::hash;

use db::request::DbConnection;
use db::models::{NewUser, Users};

#[derive(Serialize, Deserialize)]
pub struct JsonUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(
    conn: DbConnection,
    user: Json<JsonUser>,
) -> Result<status::Created<String>, Failure> {
    use db::schema::users;

    let new_user = NewUser {
        first_name: user.first_name.to_string(),
        last_name: user.last_name.to_string(),
        email: user.email.to_string(),
        password: hash(&user.password.to_string(), 8).expect("bcrypt error"),
    };

    insert_into(users::table)
        .values(&new_user)
        .get_result(&*conn)
        .map(|user: Users| {
            let url = format!("/users/{:?}", user.id);
            status::Created(url, Some("".to_string()))
        })
        .map_err(|err| match err {
            DatabaseError(UniqueViolation, _) => Failure(Status::Conflict),
            _ => Failure(Status::InternalServerError),
        })
}

#[get("/<id>")]
pub fn get(conn: DbConnection, id: i32) -> Result<Json<Value>, Failure> {
    use db::schema::users::dsl;

    dsl::users
        .filter(dsl::id.eq(&id))
        .first::<Users>(&*conn)
        .map(|user| Json(json!(user)))
        .map_err(|err| match err {
            NotFound => Failure(Status::NotFound),
            _ => Failure(Status::InternalServerError),
        })
}
