use rocket::{get, post, delete, uri, Rocket, Build};
use rocket_okapi::{JsonSchema, openapi_get_routes_spec, openapi};
use rocket::response::Redirect;
use rocket::http::ContentType;
use serde::{Deserialize, Serialize};
use rocket::form::{Form, FromForm};
use rocket_dyn_templates::context;
use sqlx::{self, Row};
use rocket_db_pools::Connection;

use libsaft::err::SaftResult;
use libsaft::template::Template;
use libsaft::docs::SaftDocsState;
use crate::db::Db;


#[openapi]
#[get("/semestermanagement")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, include_str!("../compiled-assets/templates/articles.html"))
}


pub async fn stage(rocket: Rocket<Build>) -> Rocket<Build> {
    let (routes, api_spec) = openapi_get_routes_spec![index];
    rocket.state::<SaftDocsState>().unwrap().merge(&api_spec);
    rocket.mount("/", routes)
}
