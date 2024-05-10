use rocket::serde::json::Json;
use rocket::{FromForm, get, post};
use rocket_db_pools::{Connection, Database};

use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::{JsonSchema, openapi, gen};

use crate::Db;

#[derive(Serialize, Deserialize, FromForm, Clone, sqlx::FromRow)]
struct Script {
    pk: i32,
    id: String,
    name: String,
    chair: String,
    active: bool,
}

///Get row information based on pk
#[get("/<id>")]
pub async fn read(mut db: Connection<Db>, id: i64) -> String {
    let row = sqlx::query_as::<_, Script>(
        "SELECT * FROM articles WHERE pk = ?").bind(id)
        .fetch_one(&mut **db).await;

    let entry = match row {
        Ok(row) => row,
        Err(e) => {
            println!("Error selecting row: {}", e);
            return "ERROR".to_string();
        }
    };

    return format!("id={}, name={}, chair={}, active={} \n",
                 entry.id,
                 entry.name,
                 entry.chair,
                 entry.active);
}

/*
/// Create a row entry in the arcticles table
#[openapi]
#[post("/entry", format = "json", data = "<data>")]
pub fn table_entry(data: Json<Script>) -> (){
    format!("Name:{}, Chair:{}", data.name, data.chair)
}

pub fn invoice_post(data: Json<Script>) -> SaftResult<(ContentType, Vec<u8>)> {
    let pdf = gen_pdf_from_typst_template(src, &*data)?;

    Ok((ContentType::PDF, pdf))
}
*/
