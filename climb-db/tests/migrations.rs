use common::TestDatabase;
use diesel_migrations::MigrationHarness;

mod common;

#[test]
pub fn from_scratch() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::new("test__migrations__from_scratch");

    let conn = db.connection();
    let result = conn.run_pending_migrations(climb_db::MIGRATIONS);

    assert!(result.is_ok());
}
