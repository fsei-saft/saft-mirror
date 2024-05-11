use rocket::http::ContentType;
use rocket::{launch, get};
use rocket::fairing::AdHoc;
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi, openapi_get_routes_spec};
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_db_pools::Database;
use sqlx;

use libsaft::template::Template;

mod db;
mod transactions;

#[openapi]
#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, include_str!("../compiled-assets/templates/index.html"))
}

#[launch]
fn entry() -> _ {
    sqlx::any::install_default_drivers();

    let mut rocket = rocket::build()
        .attach(db::Db::init())
        .attach(Template::fairing())
        .attach(AdHoc::try_on_ignite("SQL Migrations", db::run_migrations))
        .mount("/docs", make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("Resource", "/openapi.json")],
                ..Default::default()
            },
            ..Default::default()
        }));

    let settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs!{
        rocket, "/".to_string(), settings,
        "/" => openapi_get_routes_spec![index],
        "/" => transactions::stage(),
    };

    rocket
}
