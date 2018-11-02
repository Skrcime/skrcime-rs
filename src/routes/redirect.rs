use diesel::prelude::*;
use rocket::response::Redirect;

use db::models::Url;
use db::request::DbConnection;

#[get("/<hash>", rank = 3)]
pub fn redirect(conn: DbConnection, hash: String) -> Redirect {
    use db::schema::urls::dsl;

    let url = dsl::urls.filter(dsl::hash.eq(&hash)).first::<Url>(&*conn);
    match url {
        Ok(url) => Redirect::to(url.target),
        Err(_) => Redirect::to("/"),
    }
}
