use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel_migrations::MigrationHarness;
use std::env;

pub struct TestDatabase {
    conn: Option<PgConnection>,
    db_url: String,
    db_name: String,
}

impl TestDatabase {
    pub fn new(db_name: &str) -> Self {
        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let mut conn = PgConnection::establish(&format!("{}/postgres", database_url)).expect("Failed to establish connection with database");

        diesel::sql_query(format!("DROP DATABASE IF EXISTS {}", db_name))
            .execute(&mut conn)
            .expect("Failed to drop existing database");

        diesel::sql_query(format!("CREATE DATABASE {}", db_name))
            .execute(&mut conn)
            .expect("Failed to make database");

        let conn = PgConnection::establish(&format!("{}/{}", database_url, db_name))
            .expect("Could not connect to test database");

        TestDatabase {
            conn: Some(conn),
            db_url: database_url.to_string(),
            db_name: db_name.to_string(),
        }
    }

    #[allow(dead_code)] // TODO Why does this method show up as dead when its public?
    pub fn with_migrations(db_name: &str) -> Self {
        let mut db = TestDatabase::new(db_name);
        let conn = db.connection();
        conn.run_pending_migrations(climb_db::MIGRATIONS).expect("Failed to run pending migrations");
        db
    }

    pub fn connection(&mut self) -> &mut PgConnection {
        self.conn.as_mut().expect("Connection closed")
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        self.conn.take();

        let mut conn = PgConnection::establish(&format!("{}/postgres", self.db_url)).expect("");

        diesel::sql_query(format!("DROP DATABASE {}", self.db_name))
            .execute(&mut conn)
            .expect("Failed to drop table");
    }
}
