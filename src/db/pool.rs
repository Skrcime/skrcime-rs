use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

static DATABASE_URL: &'static str = dotenv!("DATABASE_URL");

pub fn init() -> ConnectionPool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("db pool")
}
