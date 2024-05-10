use std::ops::{Deref, DerefMut};

use rocket::request::{FromRequest, Outcome, Request};
use rocket_db_pools::Database;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

pub struct Connection<D: Database>(rocket_db_pools::Connection<D>);

#[rocket::async_trait]
impl<'r, D: Database> FromRequest<'r> for Connection<D> {
    type Error = <rocket_db_pools::Connection<D> as FromRequest<'r>>::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        rocket_db_pools::Connection::from_request(req).await.map(|v| Self(v))
    }
}

impl<'r, D: Database> OpenApiFromRequest<'r> for Connection<D> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

impl<D: Database> Deref for Connection<D> {
    type Target = <<D as rocket_db_pools::Database>::Pool as rocket_db_pools::Pool>::Connection;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<D: Database> DerefMut for Connection<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}
