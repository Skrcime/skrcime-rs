#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;
use validator::Validate;

use super::schema::urls;
use super::schema::users;

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Url {
    pub id: i32,
    pub target: String,
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Validate, Insertable, Debug)]
#[table_name = "urls"]
pub struct NewUrl {
    #[validate(url)]
    pub target: String,
    #[validate(length(min = "1"))]
    pub hash: Option<String>,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: Option<bool>,
    pub welcome: Option<bool>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Validate, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
    pub password: String,
    #[validate(url)]
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
    pub welcome: Option<bool>,
}
