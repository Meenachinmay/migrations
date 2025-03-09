use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::path::Path;

pub struct Migrator {
    pool: Pool<Postgres>,
}

impl Migrator {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        // Connect to the database
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        // This is the path to your migrations inside the crate
        let migrations_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("migrations");

        println!("Running migrations from: {:?}", migrations_path);

        // Run the migrations
        sqlx::migrate::Migrator::new(migrations_path)
            .await?
            .run(&self.pool)
            .await?;

        println!("Migrations completed successfully");
        Ok(())
    }
}