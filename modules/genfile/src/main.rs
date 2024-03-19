mod invoice;

use rocket::launch;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;

#[launch]
fn entry() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![
            invoice::invoice
        ])
        .mount("/docs", make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("Resource", "/openapi.json")],
                ..Default::default()
            },
            ..Default::default()
        }))
}
