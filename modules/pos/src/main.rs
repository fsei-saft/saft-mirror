use libsaft::db::Connection;
use libsaft::err::SaftResult;

use rocket::{launch, get};
use rocket_okapi::{openapi_get_routes, openapi};
use rocket_okapi::rapidoc::{GeneralConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("pos")]
struct Db(sqlx::SqlitePool);

#[openapi]
#[get("/")]
async fn test(mut db: Connection<Db>) -> SaftResult<()> {
    let _ = sqlx::query("SELECT content FROM logs WHERE id = ?")
        .bind(0)
        .fetch_one(&mut **db).await?;
    Ok(())
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
