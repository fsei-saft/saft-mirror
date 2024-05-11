mod db;

use rocket::launch;
use rocket_db_pools::Database;
use rocket_db_pools::sqlx;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;

use libsaft::template::Template;


#[derive(Database)]
#[database("kiwi")]
pub struct Db(sqlx::SqlitePool);



#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", openapi_get_routes![
            db::table,
            db::start,
            db::home,
            db::new,
            db::entry,
            db::edit,
            db::delete,
            db::alter
        ])
        .mount("/docs", make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("Resource", "/openapi.json")],
                ..Default::default()
            },
            ..Default::default()
        }))
        .attach(Template::fairing())
}
