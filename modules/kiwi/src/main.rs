use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

#[derive(sqlx::FromRow)]
struct Script {
    pk: i32,
    id: String,
    name: String,
    chair: String,
    active: bool,
}
const DATABASE_URL: &str = "sqlite://Database/kiwi.db";


#[tokio::main]
async fn main() {
    // Create database if not exists:
    if !Sqlite::database_exists(DATABASE_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("Database created"),
            Err(e) => println!("Error creating database: {}", e),
        }
    } else {
        println!("Database already exists");
    }
    // Connect to database:
    let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();

    // Create table (would normally do this in ./migrations/*.sql):
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS articles (
        pk INTEGER PRIMARY KEY,
        id TEXT NOT NULL,
        name TEXT NOT NULL,
        chair TEXT NOT NULL,
        active BOOLEAN NOT NULL
    )WITHOUT ROWID;").execute(&pool).await;

    match result {
        Ok(_) => println!("Table created"),
        Err(e) => println!("Error creating table: {}", e),
    }

    // Create new row to insert:
    let data = Script {
        pk: 1,
        id: "EI2032".to_string(),
        name: "Schaltungstheorie".to_string(),
        chair: "MSV".to_string(),
        active: false,
    };

    let result = sqlx::query(
        "INSERT INTO articles (pk, id, name, chair, active)
        VALUES ($1, $2, $3, $4, $5)")
        .bind(data.pk)
        .bind(data.id)
        .bind(data.name)
        .bind(data.chair)
        .bind(data.active)
        .execute(&pool).await;

    match result {
        Ok(_) => println!("Row inserted"),
        Err(e) => println!("Error inserting row: {}", e),
    }

    // Select row in database:
    let keys = sqlx::query_as::<_, Script>(
        "SELECT * FROM articles"
        ).fetch_all(&pool).await;

    let key_vec = match keys {
        Ok(keys) => keys,
        Err(e) => {
            println!("Error selecting row: {}", e);
            return;
        }
    };

    for key in key_vec {
        println!("id={}, name={}, chair={}, active={}",
                 key.id,
                 key.name,
                 key.chair,
                 key.active);
    }
}
