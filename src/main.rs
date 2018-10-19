extern crate dotenv;
extern crate skrcime;

use dotenv::dotenv;
use skrcime::rocket;

fn main() {
    dotenv().ok();
    rocket().launch();
}
