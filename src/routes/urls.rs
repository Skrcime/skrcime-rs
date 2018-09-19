use diesel;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

use rocket::http::Status;
use rocket::response::status::{Created, Custom};
use rocket_contrib::{Json, Value};

use validator::Validate;

use super::response::{error_message, error_validation, url_to_json};
use utils::random_hash;

use db::models::{NewUrl, Url};
use db::request::DbConnection;

#[post("/", format = "application/json", data = "<url>")]
pub fn create(
    conn: DbConnection,
    url: Json<NewUrl>,
) -> Result<Created<Json<Value>>, Custom<Json<Value>>> {
    if let Err(err) = url.validate() {
        return Err(error_validation(err));
    }

    let hash = random_hash().unwrap();
    println!("char is: {}", hash);
    let new_url = NewUrl {
        target: url.target.to_string(),
        hash: match url.hash {
            Some(ref hash) => Some(hash.to_string()),
            None => Some(hash),
        },
    };

    use db::schema::urls;
    diesel::insert_into(urls::table)
        .values(&new_url)
        .get_result(&*conn)
        .map(|url: Url| {
            let location = format!("/urls/{:?}", url.id);
            Created(location, Some(url_to_json(url)))
        }).map_err(|err| match err {
            DatabaseError(UniqueViolation, _) => {
                error_message(Status::Conflict, "Url with that hash already exists")
            }
            _ => {
                println!("{:?}", err);
                error_message(Status::InternalServerError, "Internal server error")
            }
        })
}
