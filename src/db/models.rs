use chrono::NaiveDateTime;

use super::schema::users;

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

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub welcome: Option<bool>,
}
