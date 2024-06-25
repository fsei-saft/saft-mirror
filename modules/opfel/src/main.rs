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


#[openapi]
#[get("/")]
async fn index()  -> SaftResult<Template> {
    Ok(Template::render("index", include_str!("../compiled-assets/templates/index.html.tera"), context![]))
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
        ("base.html", include_str!("../compiled-assets/templates/common/base.html")),
        ("nav.html", include_str!("../compiled-assets/templates/common/nav.html"))];

    let _ = rocket.state::<ContextManager>().unwrap().templates_add(common_templates);
    rocket.mount("/", routes)
}
