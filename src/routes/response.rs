use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::json::{Json, JsonValue};

use validator::ValidationErrors;

use db::models::{Url, User};

pub fn user_to_json(user: User) -> Json<JsonValue> {
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

pub fn urls_to_json(urls: Vec<Url>) -> Json<JsonValue> {
    let mut list = vec![];
    for url in &urls {
        list.push(json!({
            "id": url.id,
            "target": url.target,
            "hash": url.hash,
        }));
    }
    Json(json!(list))
}

pub fn url_to_json(url: Url) -> Json<JsonValue> {
    Json(json!({
        "id": url.id,
        "target": url.target,
        "hash": url.hash,
    }))
}

pub fn error_message(status: Status, message: &str) -> Custom<Json<JsonValue>> {
    Custom(status, Json(json!({ "message": message })))
}

pub fn error_validation(err: ValidationErrors) -> Custom<Json<JsonValue>> {
    let fields: Vec<&str> = err.inner().iter().map(|(&key, _)| key).collect();
    Custom(
        Status::BadRequest,
        Json(json!({ "message": "Validation error", "invalid_fields": fields })),
    )
}
