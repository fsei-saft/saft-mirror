use rocket::{delete, uri};
use rocket::response::Redirect;
use rocket_dyn_templates::context;
use rocket_db_pools::Connection;
use rocket::{get, post};
use rocket::form::{Form, FromForm};
use rocket_okapi::{JsonSchema, openapi_get_routes_spec, openapi};
use rocket_okapi::okapi::openapi3::OpenApi;
use sqlx::{self, Row};
use serde::{Serialize, Deserialize};

use libsaft::err::SaftResult;
use libsaft::template::Template;

use crate::db::Db;

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone)]
pub struct Transaction {
    id: i64,
    description: String
}

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone)]
pub struct CreateTransaction {
    description: String
}

#[openapi]
#[delete("/transactions/<id>")]
async fn delete(id: i64, mut db: Connection<Db>) -> SaftResult<Redirect> {
    let _ = sqlx::query("DELETE FROM transactions WHERE id = ?")
        .bind(id)
        .execute(&mut **db).await?;

    Ok(Redirect::to(uri!("/transactions")))
}

#[openapi]
#[post("/transactions", data = "<transaction>")]
async fn create(transaction: Form<CreateTransaction>, mut db: Connection<Db>) -> SaftResult<Redirect> {
    let _ = sqlx::query("INSERT INTO transactions (description) VALUES (?)")
        .bind(&transaction.description)
        .execute(&mut **db).await?;

    Ok(Redirect::to(uri!("/transactions")))
}

#[openapi]
#[get("/transactions/new")]
async fn new() -> SaftResult<Template> {
    Ok(Template::render("transactions_new.html.tera", include_str!("../compiled-assets/templates/transactions_new.html.tera"), context![]))
}

#[openapi]
#[get("/transactions")]
async fn list(mut db: Connection<Db>) -> SaftResult<Template> {
    let transactions: Vec<Transaction> = sqlx::query("SELECT id, description FROM transactions")
        .fetch_all(&mut **db).await?
        .into_iter()
        .map(|row| Transaction {
            id: row.get(0),
            description: row.get(1)
        })
        .collect();

    Ok(Template::render("transactions_list.html.tera", include_str!("../compiled-assets/templates/transactions_list.html.tera"), context![transactions]))
}

pub fn stage() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![delete, create, new, list]
}
