use std::fmt::Debug;

#[cfg(feature = "pdf")]
use ecow::EcoVec;
use rocket::{Request, Response};
use rocket::http::{ContentType, Status};
use typst::diag::SourceDiagnostic;
use typst::World;

#[cfg(feature = "web")]
use rocket::response::Responder;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::util::ensure_status_code_exists;
use crate::err::SaftErrKind::Typst;

#[derive(Debug)]
pub enum SaftErrKind {
    Typst
}

#[derive(Debug)]
pub struct SaftErr {
    kind: SaftErrKind,
    msg: String
}

#[cfg(feature = "pdf")]
impl<T: World> From<(EcoVec<SourceDiagnostic>, &T)> for SaftErr {
    fn from((diagnostics, world): (EcoVec<SourceDiagnostic>, &T)) -> Self {
        let mut msg = String::new();

        for diag in diagnostics {
            if !diag.span.is_detached() {
                let src = world.source(diag.span.id().unwrap()).unwrap();
                msg += &format!("{:?} ({}):\n  {}", src.id(), src.range(diag.span).unwrap().start, diag.message);
            } else {
                msg += &format!("unknown:\n  {}", diag.message);
            }
        }

        Self {
            kind: Typst,
            msg
        }
    }
}

#[cfg(feature = "web")]
impl<'r, 'o: 'r> Responder<'r, 'o> for SaftErr {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let file = format!(
            include_str!("../compiled-assets/html/500.html"),
            self.kind,
            self.msg
        );

        let response = (Status::InternalServerError, (ContentType::HTML, file));
        Response::build_from(response.respond_to(request)?).ok()
    }
}

#[cfg(feature = "web")]
impl OpenApiResponderInner for SaftErr {
    fn responses(_gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut responses = Responses::default();
        ensure_status_code_exists(&mut responses, 500);
        Ok(responses)
    }
}

pub type SaftResult<T> = Result<T, SaftErr>;