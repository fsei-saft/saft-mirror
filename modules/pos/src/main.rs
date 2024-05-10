use rocket::{launch, get};
use rocket_okapi::{openapi_get_routes, openapi};
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_db_pools::{sqlx, Database, Connection};

#[derive(Database)]
#[database("pos")]
struct Db(sqlx::SqlitePool);

impl<'r> OpenApiFromRequest<'r> for &'r Db {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[openapi]
#[get("/")]
async fn test(_db: &Db) -> () {
    ()
}

#[launch]
fn entry() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", openapi_get_routes![
            test
        ])
        .mount("/docs", make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("Resource", "/openapi.json")],
                ..Default::default()
            },
            ..Default::default()
        }))
}
