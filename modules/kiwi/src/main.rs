mod db;

use rocket::{Config, launch,routes};
use rocket_db_pools::Database;
use rocket_db_pools::sqlx::{self, Row};
use rocket::serde::{Deserialize, Serialize};


#[derive(Database)]
#[database("kiwi")]
pub struct Db(sqlx::SqlitePool);

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Script {
    pk: i32,
    id: String,
    name: String,
    chair: String,
    active: bool,
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::init()).mount("/", routes![db::read])
}
