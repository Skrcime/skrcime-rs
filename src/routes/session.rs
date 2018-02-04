
use diesel::prelude::*;

use rocket_contrib::{Json, Value};

use bcrypt::verify;

use db::request::DbConnection;
use db::models::{Users};
use db::schema::users::dsl::*;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[post("/", format = "application/json", data = "<cred>")]
pub fn create(conn: DbConnection, cred: Json<Credentials>) -> Json<Value> {
    let result = users
        .filter(email.eq(&cred.email.to_string()))
        .first::<Users>(&*conn);

    match result {
        Ok(user) => {
          match verify(&cred.password.to_string(), &user.password) {
            Ok(_) => Json(json!({ "token": "TODO" })),
            Err(err) => Json(json!({"error": err.to_string()})),
          }
        },
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}
