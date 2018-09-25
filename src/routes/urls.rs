use diesel;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::{Error, Error::DatabaseError};

use rocket::http::Status;
use rocket::response::status::{Created, Custom};
use rocket_contrib::{Json, Value};

use validator::Validate;

use super::response::{error_message, error_validation, url_to_json};
use super::session::Session;

use utils::random_hash;

use db::models::{NewUrl, Url, UserUrl};
use db::request::DbConnection;

#[post("/", format = "application/json", data = "<url>")]
pub fn create_user(
    session: Session,
    conn: DbConnection,
    url: Json<NewUrl>,
) -> Result<Created<Json<Value>>, Custom<Json<Value>>> {
    create(conn, url, Some(session.0))
}

#[post("/", format = "application/json", data = "<url>", rank = 2)]
pub fn create_public(
    conn: DbConnection,
    url: Json<NewUrl>,
) -> Result<Created<Json<Value>>, Custom<Json<Value>>> {
    create(conn, url, None)
}

fn create(
    conn: DbConnection,
    url: Json<NewUrl>,
    user_id: Option<i32>,
) -> Result<Created<Json<Value>>, Custom<Json<Value>>> {
    if let Err(err) = url.validate() {
        return Err(error_validation(err));
    }

    let values = NewUrl {
        target: url.target.to_string(),
        hash: match url.hash {
            Some(ref hash) => Some(hash.to_string()),
            None => Some(random_hash().unwrap()),
        },
    };

    use db::schema::{urls, user_urls};
    conn.build_transaction()
        .run::<_, Error, _>(|| {
            let result: Result<Url, Error> = diesel::insert_into(urls::table)
                .values(&values)
                .get_result(&*conn);

            if let Some(uid) = user_id {
                if let Ok(ref url) = result {
                    let user_url = UserUrl {
                        user_id: uid,
                        url_id: url.id,
                    };
                    diesel::insert_into(user_urls::table)
                        .values(&user_url)
                        .execute(&*conn)?;
                }
            }

            result
        }).map(|url: Url| {
            let location = format!("/urls/{:?}", url.id);
            Created(location, Some(url_to_json(url)))
        }).map_err(|err| match err {
            DatabaseError(UniqueViolation, _) => {
                error_message(Status::Conflict, "User with that email already exists")
            }
            _ => error_message(Status::InternalServerError, "Internal server error"),
        })
}
