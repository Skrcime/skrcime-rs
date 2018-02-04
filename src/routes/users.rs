use diesel::insert_into;
use diesel::prelude::*;

use rocket_contrib::{Json, Value};
use rocket::http::RawStr;

use bcrypt::hash;

use db::request::DbConnection;
use db::models::{NewUser, Users};
use db::schema::users::dsl::*;

#[derive(Serialize, Deserialize)]
pub struct JsonUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(conn: DbConnection, user: Json<JsonUser>) -> Json<Value> {
    use db::schema::users;

    let hashed_password = hash(&user.password.to_string(), 8).expect("bcrypt error");
    let new_user = NewUser {
        first_name: user.first_name.to_string(),
        last_name: user.last_name.to_string(),
        email: user.email.to_string(),
        password: hashed_password,
    };

    let result: QueryResult<Users> = insert_into(users::table)
        .values(&new_user)
        .get_result(&*conn);

    match result {
        Ok(user) => Json(json!(user)),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

#[get("/<remail>", format = "application/json")]
pub fn get_by_email(conn: DbConnection, remail: &RawStr) -> Json<Value> {
    let result = users
        .filter(email.eq(&remail.as_str()))
        .first::<Users>(&*conn);

    match result {
        Ok(user) => Json(json!(user)),
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}
