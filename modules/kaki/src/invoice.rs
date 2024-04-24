use include_json::{include_json, IncludeJson};
use rocket::serde::json::Json;
use rocket::{FromForm, get, post};
use rocket::http::ContentType;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::{JsonSchema, openapi};
use libsaft::err::SaftResult;
use libsaft::pdf::gen_pdf_from_typst_template;

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone, IncludeJson)]
#[schemars(example = "InvoiceInfo::default")]
pub struct InvoiceInfo {
    number: String,
    recipient: InvoiceRecipient,
    issuer: String,
    date: String,
    total: f64,
    items: Vec<InvoiceItem>,
    taxes: Vec<InvoiceTax>
}

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone, IncludeJson)]
pub struct InvoiceRecipient {
    address: String,
}

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone, IncludeJson)]
pub struct InvoiceItem {
    quantity: i32,
    description: String,
    number: String,
    netto: f64,
    brutto: f64,
    tax_rate: f64,
    total: f64
}

#[derive(Serialize, Deserialize, FromForm, JsonSchema, Clone, IncludeJson)]
pub struct InvoiceTax {
    rate: f64,
    description: String,
    total: f64
}

impl Default for InvoiceInfo {
    fn default() -> Self { include_json!(InvoiceInfo, "../compiled-assets/json/invoice.json") }
}

/// Generate a default invoice as a PDF
#[openapi]
#[get("/invoice")]
pub fn invoice_get() -> SaftResult<(ContentType, Vec<u8>)> {
    invoice_post(Json::from(InvoiceInfo::default()))
}

/// Generate an invoice as a PDF from an [InvoiceInfo]
#[openapi]
#[post("/invoice", format = "json", data = "<data>")]
pub fn invoice_post(data: Json<InvoiceInfo>) -> SaftResult<(ContentType, Vec<u8>)> {
    let src = String::from(include_str!("../compiled-assets/typst-templates/invoice.typ"));
    let pdf = gen_pdf_from_typst_template(src, &*data)?;

    Ok((ContentType::PDF, pdf))
}
