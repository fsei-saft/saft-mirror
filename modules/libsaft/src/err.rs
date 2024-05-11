use std::fmt::Debug;

#[cfg(feature = "pdf")]
use ecow::EcoVec;
#[cfg(feature = "pdf")]
use typst::diag::SourceDiagnostic;
#[cfg(feature = "pdf")]
use typst::World;
#[cfg(feature = "web")]
use rocket::{Request, Response};
#[cfg(feature = "web")]
use rocket::http::{ContentType, Status};
#[cfg(feature = "web")]
use rocket::response::Responder;
#[cfg(feature = "web")]
use rocket_okapi::response::OpenApiResponderInner;
#[cfg(feature = "web")]
use rocket_okapi::gen::OpenApiGenerator;
#[cfg(feature = "web")]
use rocket_okapi::okapi::openapi3::Responses;
#[cfg(feature = "web")]
use rocket_okapi::util::ensure_status_code_exists;
#[cfg(feature = "web")]
use rocket_db_pools::sqlx;

#[derive(Debug)]
pub enum SaftErrKind {
    Typst,
    Db,
    ResourceNotFound
}

impl Into<Status> for SaftErrKind {
    fn into(self) -> Status {
        match self {
            Self::Typst => Status::InternalServerError,
            Self::Db => Status::InternalServerError,
            Self::ResourceNotFound => Status::NotFound
        }
    }
}

#[derive(Debug)]
pub struct SaftErr {
    kind: SaftErrKind,
    msg: String
}

impl From<SaftErrKind> for SaftErr {
    fn from(kind: SaftErrKind) -> Self {
        Self {
            kind,
            msg: String::new()
        }
    }
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
            kind: SaftErrKind::Typst,
            msg
        }
    }
}

#[cfg(feature = "web")]
impl From<sqlx::Error> for SaftErr {
    fn from(e: sqlx::Error) -> Self {
        Self {
            kind: SaftErrKind::Db,
            msg: format!("{}", e)
        }
    }
}

#[cfg(feature = "web")]
impl<'r, 'o: 'r> Responder<'r, 'o> for SaftErr {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let file = format!(
            include_str!("../compiled-assets/html/error.html"),
            self.kind,
            self.msg
        );

        let response: (Status, (ContentType, String)) = (self.kind.into(), (ContentType::HTML, file));
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
