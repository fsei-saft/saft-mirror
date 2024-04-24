use rocket::get;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::openapi;


pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![hello]
}

#[openapi]
#[get("/opfel")]
pub fn hello() -> &'static str {
    "Hello, OPFEL!"
}
