use diesel;
use diesel::prelude::*;
use diesel::result::Error::{DatabaseError, NotFound};
use diesel::result::DatabaseErrorKind::UniqueViolation;

use rocket::response::{status, Failure};
use rocket::http::Status;
use rocket_contrib::{Json, Value};

use bcrypt::hash;

use db::request::DbConnection;
use db::models::{NewUser, UpdateUser, User};

#[post("/", format = "application/json", data = "<user>")]
pub fn create(
    conn: DbConnection,
    user: Json<NewUser>,
) -> Result<status::Created<Json<Value>>, Failure> {
    use db::schema::users;

    let new_user = NewUser {
        first_name: user.first_name.to_string(),
        last_name: user.last_name.to_string(),
        avatar_url: match user.avatar_url {
            Some(ref avatar) => Some(avatar.to_string()),
            None => None,
        },
        email: user.email.to_string(),
        password: hash(&user.password.to_string(), 8).expect("bcrypt error"),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&*conn)
        .map(|user: User| {
            let location = format!("/users/{:?}", user.id);
            status::Created(location, Some(user_to_json(user)))
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
        .first::<User>(&*conn)
        .map(user_to_json)
        .map_err(|err| match err {
            NotFound => Failure(Status::NotFound),
            _ => Failure(Status::InternalServerError),
        })
}

#[patch("/<id>", format = "application/json", data = "<user>")]
pub fn update(conn: DbConnection, id: i32, user: Json<UpdateUser>) -> Result<Json<Value>, Failure> {
    use db::schema::users::dsl;

    diesel::update(dsl::users.filter(dsl::id.eq(&id)))
        .set(&user.into_inner())
        .get_result(&*conn)
        .map(|user: User| Json(json!(user)))
        .map_err(|err| match err {
            DatabaseError(UniqueViolation, _) => Failure(Status::Conflict),
            _ => Failure(Status::InternalServerError),
        })
}

fn user_to_json(user: User) -> Json<Value> {
    Json(json!({
        "id": user.id,
        "first_name": user.first_name,
        "last_name": user.last_name,
        "email": user.email,
        "admin": user.admin,
        "welcome": user.welcome,
        "avatar_url": user.avatar_url,
        "created_at": user.created_at,
    }))
}
