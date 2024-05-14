use rocket::{fairing::{self, Fairing, Info, Kind}, get, http::ContentType, Build, Rocket};
use rocket_okapi::{openapi, openapi_get_routes_spec};

use crate::err::{SaftErrKind, SaftResult};
use crate::docs::SaftDocsState;

pub struct SaftResources;

#[openapi]
#[get("/resources/<file>")]
async fn resources(file: String) -> SaftResult<(ContentType, &'static str)> {
    match file.as_str() {
        "htmx.min.js" => Ok((ContentType::JavaScript, include_str!("../compiled-assets/resources/htmx.min.js"))),
        _ => Err(SaftErrKind::ResourceNotFound.into())
    }
}

#[rocket::async_trait]
impl Fairing for SaftResources {
    fn info(&self) -> Info {
        let kind = Kind::Ignite | Kind::Liftoff;

        Info { kind, name: "Static resources" }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let (routes, api_spec) = openapi_get_routes_spec![resources];
        rocket.state::<SaftDocsState>().unwrap().merge(&api_spec);
        let rocket = rocket.mount("/", routes);
        Ok(rocket)
    }
}
