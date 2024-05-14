use libsaft::docs::{SaftDocsEnd, SaftDocsStart, SaftDocsState};
use libsaft::resources::SaftResources;
use rocket::http::ContentType;
use rocket::{launch, get};
use rocket::fairing::AdHoc;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_db_pools::Database;

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
    rocket::build()
        .attach(SaftDocsStart)
        .attach(db::Db::init())
        .attach(AdHoc::try_on_ignite("SQL migrations", db::run_migrations))
        .attach(Template::fairing())
        .attach(SaftResources)
        .attach(AdHoc::on_ignite("Routes: index", |r| async move {
            let (routes, api_spec) = openapi_get_routes_spec![index];
            r.state::<SaftDocsState>().unwrap().merge(&api_spec);
            r.mount("/", routes)
        }))
        .attach(AdHoc::on_ignite("Routes: transactions", transactions::stage))
        .attach(SaftDocsEnd)
}
