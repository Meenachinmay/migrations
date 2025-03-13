use dotenv::dotenv;
use std::env;
// This imports the Migrator from your own lib.rs
use migrations::Migrator;

#[tokio::main]
async fn main() {
    // Load .env file to get DATABASE_URL
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("DATABASE_URL environment variable not set");
            std::process::exit(1);
        }
    };

    println!("Running migrations against: {}", database_url);

    // Create the migrator with your database connection
    let migrator = match Migrator::new(&database_url).await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Run migrations
    if let Err(e) = migrator.run_migrations().await {
        eprintln!("Migration failed: {}", e);
        std::process::exit(1);
    }

    println!("Migrations completed successfully");
}