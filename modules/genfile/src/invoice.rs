use rocket::{FromForm, get};
use rocket::http::ContentType;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::{JsonSchema, openapi};
use libsaft::err::SaftResult;
use libsaft::pdf::gen_pdf_from_typst_template;

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone)]
pub struct InvoiceInfo {
    number: String,
    recipient: InvoiceRecipient,
    issuer: String,
    items: Vec<InvoiceItem>
}

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone)]
pub struct InvoiceRecipient {
    address: String,
}

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone)]
pub struct InvoiceItem {
    amount: i32,
    description: String,
    netto: f64,
    brutto: f64,
    tax_rate: f64
}

impl Default for InvoiceInfo {
    fn default() -> Self { serde_json::from_str(include_str!("../compiled-assets/json/invoice.json")).unwrap() }
}

#[openapi]
#[get("/invoice?<data>")]
pub fn invoice(data: Option<InvoiceInfo>) -> SaftResult<(ContentType, Vec<u8>)> {
    let src = String::from(include_str!("../compiled-assets/typst-templates/invoice.typ"));
    let pdf = gen_pdf_from_typst_template(src, &data.unwrap_or_default())?;

    Ok((ContentType::PDF, pdf))
}
