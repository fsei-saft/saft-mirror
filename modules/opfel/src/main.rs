use libsaft::docs::{SaftDocsEnd, SaftDocsStart, SaftDocsState};
use libsaft::resources::SaftResources;

use rocket::{launch, get, Build, Rocket};
use rocket::fairing::AdHoc;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_db_pools::Database;
use rocket_dyn_templates::context;

use libsaft::template::{Template,ContextManager};
use libsaft::err::SaftResult;
use sqlx;

mod db;
mod articles;

const HTML: &str = "../compiled-assets/templates/";

#[openapi]
#[get("/")]
async fn index()  -> SaftResult<Template> {
    Ok(Template::render("index", include_str!(concat!(HTML, "index.html.tera")), context![]))
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
        .attach(AdHoc::on_ignite("Routes: Main", main_stage))
        .attach(AdHoc::on_ignite("Routes: articles", articles::stage))
        .attach(SaftDocsEnd)
}


async fn main_stage(rocket: Rocket<Build>) -> Rocket<Build> {
    let (routes, api_spec) = openapi_get_routes_spec![index];
    rocket.state::<SaftDocsState>().unwrap().merge(&api_spec);
    let common_templates = vec![
        ("base.html", include_str!(concat!(HTML, "base.html"))),
        ("nav.html.tera", include_str!(concat!(HTML, "nav.html")))];

    rocket.state::<ContextManager>().unwrap().templates_add(common_templates);
    rocket.mount("/", routes)
}
