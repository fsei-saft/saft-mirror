use std::fmt::format;

use libsaft::err::SaftResult;
use libsaft::template::Template;
use rocket_dyn_templates::context;
use crate::Db;


use rocket::{http::ContentType, get, post, FromForm, uri};
use rocket::form::Form;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::{JsonSchema, openapi};
use rocket::response::Redirect;



#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone, sqlx::FromRow)]
pub struct Script {
    id: String,
    name: String,
    chair: String,
    active: bool
}

#[openapi]
#[get("/")]
pub async fn start() -> Redirect {
    Redirect::to(uri!(home))
}

#[openapi]
#[get("/home")]
pub async fn home() -> SaftResult<Template> {
    Ok(Template::render("home",include_str!{"../compiled-assets/templates/home.html.tera"} ,context! {}))
}

///Get row information based on pk
#[openapi]
#[get("/table")]
pub async fn table(mut db: rocket_db_pools::Connection<Db>) -> SaftResult<Template> {
    let entry = sqlx::query_as::<_, Script>(
        "SELECT * FROM articles"
        ).fetch_all(&mut **db).await?;
        Ok(Template::render("db_list",include_str!{"../compiled-assets/templates/db_list.html.tera"} ,context! { articles: entry }))
}

/// Row entry Form
#[openapi]
#[get("/new")]
pub async fn new() -> SaftResult<Template> {
    Ok(Template::render("entry",include_str!{"../compiled-assets/templates/entry.html.tera"} ,context! {}))
}

/// push entry into DB
#[openapi]
#[post("/new", data = "<script>")]
pub async fn entry(script: Form<Script>, mut db: rocket_db_pools::Connection<Db>) -> SaftResult<Redirect> {
    sqlx::query(
        "INSERT INTO articles    (id, name, chair, active)
        VALUES ($1, $2, $3, $4)")
        .bind(&script.id)
        .bind(&script.name)
        .bind(&script.chair)
        .bind(&script.active)
        .execute(&mut **db).await?;
    Ok(Redirect::to(uri!(table)))
}

#[openapi]
#[post("/alter", data = "<id>")]
pub async fn alter(id: Form<String>, mut db: rocket_db_pools::Connection<Db>) -> SaftResult<Template> {
    let entry = sqlx::query_as::<_, Script>(
         "SELECT * FROM articles WHERE id=?").bind(id.to_string()).fetch_one(&mut **db).await?;
         Ok(Template::render("edit",include_str!{"../compiled-assets/templates/edit.html.tera"} ,context! { article: entry }))
}

//Edit an Entry in the DB
#[openapi]
#[post("/edit", data = "<script>")]
pub async fn edit(script: Form<Script>, mut db: rocket_db_pools::Connection<Db>) -> SaftResult<Redirect> {
    sqlx::query(
        "UPDATE articles SET id =$1, name=$2, chair=$3, active=$4")
        .bind(&script.id)
        .bind(&script.name)
        .bind(&script.chair)
        .bind(&script.active)
    .execute(&mut **db).await?;
     Ok(Redirect::to(uri!(table)))
}

#[openapi]
#[post("/delete", data = "<id>")]
pub async fn delete(id: Form<String>, mut db: rocket_db_pools::Connection<Db>) -> SaftResult<Redirect> {
    sqlx::query(
        "DELETE FROM articles WHERE id=?").bind(id.to_string()).execute(&mut **db).await?;
     Ok(Redirect::to(uri!(table)))
}
