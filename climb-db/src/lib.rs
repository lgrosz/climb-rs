use diesel_migrations::{EmbeddedMigrations,embed_migrations};

pub mod models;
pub mod schema;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
