use rocket::{Rocket, Build, error};
use rocket::fairing;
use rocket_db_pools::Database;
use sqlx::AnyPool;

#[derive(Database)]
#[database("feigen")]
pub struct Db(AnyPool);

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    sqlx::any::install_default_drivers();
    let db = if let Some(db) = Db::fetch(&rocket) { db } else { return Err(rocket) };
    match sqlx::migrate!("compiled-assets/migrations").run(&**db).await {
        Ok(_) => Ok (rocket),
        Err(e) => {
            error!("Failed to initialize database: {}", e);
            Err(rocket)
        }
    }
}
