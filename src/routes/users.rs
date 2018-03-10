use diesel;
use diesel::prelude::*;
use diesel::result::Error::{DatabaseError, NotFound};
use diesel::result::DatabaseErrorKind::UniqueViolation;

use rocket::response::Failure;
use rocket::response::status::{Created, Custom};
use rocket::http::Status;
use rocket_contrib::{Json, Value};

use validator::Validate;

use bcrypt::hash;

use super::session::Session;
use super::response::{error_message, error_validation, user_to_json};

use db::request::DbConnection;
use db::models::{NewUser, UpdateUser, User};

#[post("/", format = "application/json", data = "<user>")]
pub fn create(
    conn: DbConnection,
    user: Json<NewUser>,
) -> Result<Created<Json<Value>>, Custom<Json<Value>>> {
    if let Err(err) = user.validate() {
        return Err(error_validation(err));
    }

    let new_user = NewUser {
        name: user.name.to_string(),
        avatar_url: match user.avatar_url {
            Some(ref avatar) => Some(avatar.to_string()),
            None => None,
        },
        email: user.email.to_string(),
        password: hash(&user.password.to_string(), 8).expect("bcrypt error"),
    };

    use db::schema::users;
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&*conn)
        .map(|user: User| {
            let location = format!("/users/{:?}", user.id);
            Created(location, Some(user_to_json(user)))
        })
        .map_err(|err| match err {
            DatabaseError(UniqueViolation, _) => {
                error_message(Status::Conflict, "User with that email already exists")
            }
            _ => error_message(Status::InternalServerError, "Internal server error"),
        })
}

#[get("/me")]
pub fn get(session: Session, conn: DbConnection) -> Result<Json<Value>, Custom<Json<Value>>> {
    use db::schema::users::dsl;

    dsl::users
        .filter(dsl::id.eq(&session.0))
        .first::<User>(&*conn)
        .map(user_to_json)
        .map_err(|err| match err {
            NotFound => error_message(Status::NotFound, "User not found"),
            _ => error_message(Status::InternalServerError, "Internal server error"),
        })
}
#[get("/me", rank = 2)]
pub fn get_401() -> Failure {
    Failure(Status::Unauthorized)
}

#[patch("/me", format = "application/json", data = "<user>")]
pub fn update(
    session: Session,
    conn: DbConnection,
    mut user: Json<UpdateUser>,
) -> Result<Json<Value>, Custom<Json<Value>>> {
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
            DatabaseError(UniqueViolation, _) => {
                error_message(Status::Conflict, "User with that email already exists")
            }
            _ => error_message(Status::InternalServerError, "Internal server error"),
        })
}
#[patch("/me", rank = 2)]
pub fn update_401() -> Failure {
    Failure(Status::Unauthorized)
}
