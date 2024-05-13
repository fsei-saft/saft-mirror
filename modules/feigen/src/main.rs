use rocket::http::ContentType;
use rocket::{launch, get};
use rocket::fairing::AdHoc;
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi, openapi_get_routes_spec};
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_db_pools::Database;

use libsaft::template::Template;
use libsaft::err::{SaftResult, SaftErrKind};

mod db;
mod transactions;

#[openapi]
#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, include_str!("../compiled-assets/templates/index.html"))
}

#[openapi]
#[get("/resources/<file>")]
async fn resources(file: String) -> SaftResult<(ContentType, &'static str)> {
    match file.as_str() {
        "htmx.min.js" => Ok((ContentType::JavaScript, include_str!("../compiled-assets/resources/htmx.min.js"))),
        _ => Err(SaftErrKind::ResourceNotFound.into())
    }
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
        "/" => openapi_get_routes_spec![index, resources],
        "/" => transactions::stage(),
    };

    rocket
}
