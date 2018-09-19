use std::ops::Deref;

use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use diesel::pg::PgConnection;

use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use routes::session::{Session, COOKIE_KEY};

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConnection, ()> {
        let pool = request.guard::<State<ConnectionPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Session, ()> {
        req.cookies()
            .get_private(COOKIE_KEY)
            .and_then(|cookie| cookie.value().parse().ok())
            .map(Session)
            .or_forward(())
    }
}
