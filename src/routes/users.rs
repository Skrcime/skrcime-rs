use diesel;
use diesel::prelude::*;
use diesel::result::Error::{DatabaseError, NotFound};
use diesel::result::DatabaseErrorKind::UniqueViolation;

use rocket::response::{status, Failure};
use rocket::http::Status;
use rocket_contrib::{Json, Value};

use bcrypt::hash;

use super::session::Session;
use db::request::DbConnection;
use db::models::{NewUser, UpdateUser, User};

#[post("/", format = "application/json", data = "<user>")]
pub fn create(
    conn: DbConnection,
    user: Json<NewUser>,
) -> Result<status::Created<Json<Value>>, Failure> {
    use db::schema::users;

    let new_user = NewUser {
        name: user.name.to_string(),
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

#[get("/me")]
pub fn get(conn: DbConnection, session: Session) -> Result<Json<Value>, Failure> {
    use db::schema::users::dsl;

    dsl::users
        .filter(dsl::id.eq(&session.0))
        .first::<User>(&*conn)
        .map(user_to_json)
        .map_err(|err| match err {
            NotFound => Failure(Status::NotFound),
            _ => Failure(Status::InternalServerError),
        })
}
#[get("/me", rank = 2)]
pub fn get_401() -> Failure {
    Failure(Status::Unauthorized)
}

#[patch("/me", format = "application/json", data = "<user>")]
pub fn update(conn: DbConnection, session: Session, mut user: Json<UpdateUser>) -> Result<Json<Value>, Failure> {
    use db::schema::users::dsl;

    user.password = match user.password {
        Some(ref password) => Some(hash(&password.to_string(), 8).expect("bcrypt error")),
        None => None,
    };

    diesel::update(dsl::users.filter(dsl::id.eq(&session.0)))
        .set(&user.into_inner())
        .get_result(&*conn)
        .map(|user: User| Json(json!(user)))
        .map_err(|err| match err {
            DatabaseError(UniqueViolation, _) => Failure(Status::Conflict),
            _ => Failure(Status::InternalServerError),
        })
}
#[patch("/me", rank = 2)]
pub fn update_401() -> Failure {
    Failure(Status::Unauthorized)
}

fn user_to_json(user: User) -> Json<Value> {
    Json(json!({
        "id": user.id,
        "name": user.name,
        "email": user.email,
        "admin": user.admin,
        "welcome": user.welcome,
        "avatar_url": user.avatar_url,
        "created_at": user.created_at,
    }))
}
