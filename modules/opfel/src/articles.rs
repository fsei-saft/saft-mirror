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
    chair: String,
    active: bool
}

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone)]
pub struct CreateScript {
    p_id: String,
    name: String,
    chair: String,
    active: bool
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
    let rows = sqlx::query("SELECT id, product_id, name, chair, active FROM articles").fetch_all(&mut **db).await.unwrap();
    let mut articles:Vec<Script> = vec![];
    for r in rows {
        let mut ac = false;
        let i:i32 = r.get(4);
        if i==1 {ac = true;}
        let script:Script = Script {
            id: r.get(0),
            p_id: r.get(1),
            name: r.get(2),
            chair:r.get(3) ,
            active: ac};
        articles.push(script);
    }
    Ok(Template::render("articles/entries", include_str!("../compiled-assets/templates/articles/entries.html.tera"), context![articles]))
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
    let mut ac:i32 = 0;
    if script.active {ac = 1;}
    sqlx::query(
        "INSERT INTO articles (product_id, name, chair, active) VALUES ($1, $2, $3, $4)")
        .bind(&script.p_id)
        .bind(&script.name)
        .bind(&script.chair)
        .bind(ac)
        .execute(&mut **db).await?;
    Ok(Redirect::to(uri!(index)))
}

//Edit an Entry in the DB
#[openapi]
#[post("/edit", data = "<script>")]
pub async fn edit(script: Form<Script>, mut db: Connection<Db>) -> SaftResult<Redirect> {
    sqlx::query(
        "UPDATE articles SET product_id =$1, name=$2, chair=$3, active=$4")
        .bind(&script.p_id)
        .bind(&script.name)
        .bind(&script.chair)
        .bind(&script.active)
        .execute(&mut **db).await?;
     Ok(Redirect::to(uri!(index)))
}

#[openapi]
#[delete("/articles/<id>")]
async fn delete(id: i64, mut db: Connection<Db>) -> SaftResult<Redirect> {
    let _ = sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(id)
        .execute(&mut **db).await?;
    Ok(Redirect::to(uri!(index)))
}


pub async fn stage(rocket: Rocket<Build>) -> Rocket<Build> {
    let (routes, api_spec) = openapi_get_routes_spec![index, list, new, entry, edit, delete];
    rocket.state::<SaftDocsState>().unwrap().merge(&api_spec);
    rocket.mount("/", routes)
}
