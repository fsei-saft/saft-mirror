use rocket::{get, post, delete, uri, Rocket, Build};
use rocket_okapi::{JsonSchema, openapi_get_routes_spec, openapi};
use rocket::response::Redirect;
use serde::{Deserialize, Serialize};
use rocket::form::{Form, FromForm};
use rocket_dyn_templates::context;
use sqlx::{self, Row};
use rocket_db_pools::Connection;

use libsaft::err::SaftResult;
use libsaft::template::Template;
use libsaft::docs::SaftDocsState;
use crate::db::Db;

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone)]
pub struct Script {
    id:   i64,
    p_id: String,
    name: String,
    chair: String
}

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone)]
pub struct CreateScript {
    p_id: String,
    name: String,
    chair: String
}


#[openapi]
#[get("/articles")]
async fn index() -> SaftResult<Template> {
    Ok(Template::render("articles/index", include_str!("../compiled-assets/templates/articles/articles.html.tera"), context![]))
}

///Get row information based on pk
#[openapi]
#[get("/articles/list")]
pub async fn list(mut db: Connection<Db>) -> SaftResult<Template> {
    let articles: Vec<Script> = sqlx::query("SELECT id, product_id, name, chair FROM articles")
        .fetch_all(&mut **db).await?
        .into_iter()
        .map(|row| Script {
            id: row.get(0),
            p_id: row.get(1),
            name: row.get(2),
            chair:row.get(3)
        })
        .collect();
        Ok(Template::render("articles/entry", include_str!("../compiled-assets/templates/articles/entry.html.tera"), context![articles]))
}

/// Row entry Form
#[openapi]
#[get("/articles/new")]
pub async fn new() -> SaftResult<Template> {
    Ok(Template::render("articles/new",include_str!{"../compiled-assets/templates/articles/new.html.tera"} ,context! {}))
}

/// push entry into DB
#[openapi]
#[post("/articles", data = "<script>")]
pub async fn entry(script: Form<CreateScript>, mut db: Connection<Db>) -> SaftResult<Redirect> {
    sqlx::query(
        "INSERT INTO articles    (product_id, name, chair)
        VALUES ($1, $2, $3)")
        .bind(&script.p_id)
        .bind(&script.name)
        .bind(&script.chair)
        .execute(&mut **db).await?;
    Ok(Redirect::to(uri!("/articles/list")))
}

//Edit an Entry in the DB
#[openapi]
#[post("/edit", data = "<script>")]
pub async fn edit(script: Form<Script>, mut db: Connection<Db>) -> SaftResult<Redirect> {
    sqlx::query(
        "UPDATE articles SET product_id =$1, name=$2, chair=$3")
        .bind(&script.p_id)
        .bind(&script.name)
        .bind(&script.chair)
    .execute(&mut **db).await?;
     Ok(Redirect::to(uri!(index)))
}

#[openapi]
#[delete("/articles/<id>")]
async fn delete(id: i64, mut db: Connection<Db>) -> SaftResult<Redirect> {
    let _ = sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(id)
        .execute(&mut **db).await?;

    Ok(Redirect::to(uri!("/articles/list")))
}


pub async fn stage(rocket: Rocket<Build>) -> Rocket<Build> {
    let (routes, api_spec) = openapi_get_routes_spec![index, list, new, entry, edit, delete];
    rocket.state::<SaftDocsState>().unwrap().merge(&api_spec);
    rocket.mount("/", routes)
}
