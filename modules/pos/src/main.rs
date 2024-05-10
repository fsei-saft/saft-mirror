use std::collections::HashMap;

use libsaft::err::SaftResult;
use libsaft::template::Template;

use rocket::{launch, get};
use rocket_okapi::{openapi_get_routes, openapi};
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_db_pools::{sqlx, Database, Connection};

#[derive(Database)]
#[database("pos")]
struct Db(sqlx::SqlitePool);

#[openapi]
#[get("/")]
async fn test(mut db: Connection<Db>) -> SaftResult<Template> {
    let _ = sqlx::query("SELECT content FROM logs WHERE id = ?")
        .bind(0)
        .fetch_one(&mut **db).await;

    let ctx: HashMap<String, String> = HashMap::new();
    Ok(Template::render("index.html", include_str!("../compiled-assets/templates/index.html"), ctx))
}

#[launch]
fn entry() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(Template::fairing())
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
