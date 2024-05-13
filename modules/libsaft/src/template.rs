use std::sync::RwLock;
use std::ops::{Deref, DerefMut};
use rocket::fairing;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Rocket, Build, Orbit};
use rocket::{info, error_};
use rocket::log::PaintExt;
use rocket::yansi::Paint;
use rocket::response::Responder;
use rocket::response;
use rocket::request::Request;
use rocket::figment::{value::Value, error::Error};
use rocket::http::{ContentType, Status};
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use tera::Tera;
use tera;
use serde::Serialize;

#[derive(Debug)]
pub struct Template {
    name: &'static str,
    source: &'static str,
    value: Result<Value, Error>
}

impl Template {
    pub fn fairing() -> impl Fairing {
        TemplateFairing
    }

    pub fn render<C>(name: &'static str, source: &'static str, context: C) -> Template
    where
        C: Serialize
    {
        Template {
            name,
            source,
            value: Value::serialize(context),
        }
    }

    pub fn finalise(self, ctx: &mut Context) -> Result<(ContentType, String), Status> {
        if !ctx.engine.get_template_names().any(|v| v == self.name) {
            ctx.engine.add_raw_template(self.name, self.source).map_err(|_| {
                error_!("Failed to parse template \"{}\".", self.name);
                Status::InternalServerError
            })?;
        }

        let value = self.value.map_err(|_| {
            error_!("Failed to serialise template context.");
            Status::InternalServerError
        })?;

        let value = tera::Context::from_serialize(value).map_err(|_| {
            error_!("Failed to serialise template context.");
            Status::InternalServerError
        })?;

        let rendered = ctx.engine.render(self.name, &value).map_err(|_| {
            error_!("Failed to render template \"{}\".", self.name);
            Status::InternalServerError
        })?;

        Ok((ContentType::HTML, rendered))
    }
}

impl<'r> Responder<'r, 'static> for Template {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let ctx = req.rocket().state::<ContextManager>().ok_or(Status::InternalServerError)?;

        self.finalise(&mut ctx.context_mut())?.respond_to(req)
    }
}

impl OpenApiResponderInner for Template {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        String::responses(gen)
    }
}

pub struct TemplateFairing;

#[rocket::async_trait]
impl Fairing for TemplateFairing {
    fn info(&self) -> Info {
        let kind = Kind::Ignite | Kind::Liftoff;

        Info { kind, name: "Static templating" }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        Ok(rocket.manage(ContextManager::new(Context::default())))
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let _cm = rocket.state::<ContextManager>().expect("static templating ContextManager not registered in on_ignite");

        info!("{}{}", "📐 ".emoji(), "Static templating".magenta());
    }
}

#[derive(Default)]
pub struct Context {
    pub engine: Tera
}

pub struct ContextManager {
    context: RwLock<Context>
}

impl ContextManager {
    pub fn new(ctx: Context) -> ContextManager {
        ContextManager {
            context: RwLock::new(ctx)
        }
    }

    pub fn context(&self) -> impl Deref<Target=Context> + '_ {
        self.context.read().unwrap()
    }

    fn context_mut(&self) -> impl DerefMut<Target=Context> + '_ {
        self.context.write().unwrap()
    }
}
