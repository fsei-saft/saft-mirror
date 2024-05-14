use std::sync::{Arc, Mutex, MutexGuard};
use rocket::{fairing::{self, Fairing, Info, Kind}, Build, Rocket};
use rocket_okapi::{get_openapi_route, rapidoc::{make_rapidoc, GeneralConfig, RapiDocConfig}, settings::UrlObject};
use rocket_okapi::okapi::{openapi3::OpenApi, merge::merge_specs};

pub struct SaftDocsStart;

#[rocket::async_trait]
impl Fairing for SaftDocsStart {
    fn info(&self) -> Info {
        let kind = Kind::Ignite | Kind::Liftoff;

        Info { kind, name: "OpenAPI documentation resources" }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        Ok(rocket.manage(SaftDocsState::default()))
    }
}

pub struct SaftDocsEnd;

#[rocket::async_trait]
impl Fairing for SaftDocsEnd {
    fn info(&self) -> Info {
        let kind = Kind::Ignite | Kind::Liftoff;

        Info { kind, name: "OpenAPI documentation renderer" }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let rocket = rocket.mount("/docs", make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("Resource", "/openapi.json")],
                    ..Default::default()
                },
                ..Default::default()
            }));

        let settings = rocket_okapi::settings::OpenApiSettings::default();
        let api_spec = rocket.state::<SaftDocsState>().expect("OpenAPI documentation ended without being started").get().clone();
        let rocket = rocket.mount("/", vec![get_openapi_route(api_spec, &settings)]);

        Ok(rocket)
    }
}

#[derive(Clone)]
pub struct SaftDocsState {
    state: Arc<Mutex<OpenApi>>
}

impl SaftDocsState {
    pub fn get(&self) -> MutexGuard<OpenApi> {
        self.state.lock().unwrap()
    }

    pub fn merge(&self, api_spec: &OpenApi) {
        let mut state = self.state.lock().unwrap();
        merge_specs(&mut state, &"/", api_spec).unwrap();
    }
}

impl Default for SaftDocsState {
    fn default() -> Self {
        SaftDocsState {
            state: Arc::new(Mutex::new(OpenApi::new()))
        }
    }
}
