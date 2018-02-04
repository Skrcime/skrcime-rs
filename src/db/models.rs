use uuid::Uuid;
use chrono::NaiveDateTime;

use super::schema::users;

#[derive(Queryable, Serialize, Debug)]
pub struct Users {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub admin: Option<bool>,
    pub welcome: Option<bool>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
