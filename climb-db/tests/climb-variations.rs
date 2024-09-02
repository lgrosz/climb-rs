use common::TestDatabase;
use diesel::prelude::*;

mod common;

/// Tests the no_self_reference constraint
#[test]
fn no_self_reference() {
    let mut db = TestDatabase::with_migrations("test__climb_variation__no_self_reference");
    let conn = db.connection();

    use climb_db::models::{Climb, NewClimb};
    use climb_db::schema::climbs;

    let climb = diesel::insert_into(climbs::table)
        .values(NewClimb {
            names: vec![Some("The Cheat".to_string())],
        })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::models::NewClimbVariation;
    use climb_db::schema::climb_variations;

    let result = diesel::insert_into(climb_variations::table)
        .values(NewClimbVariation {
            root_id: climb.id,
            variation_id: climb.id,
        })
        .execute(conn);

    assert!(result.is_err());
}

/// Tests the root_id foreign key constraint
#[test]
fn root_fk() {
    let mut db = TestDatabase::with_migrations("test__climb_variation__root_fk");
    let conn = db.connection();

    use climb_db::models::{Climb, NewClimb};
    use climb_db::schema::climbs;

    let climb = diesel::insert_into(climbs::table)
        .values(NewClimb {
            names: vec![Some("The Cheat".to_string())],
        })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::models::NewClimbVariation;
    use climb_db::schema::climb_variations;

    let result = diesel::insert_into(climb_variations::table)
        .values(NewClimbVariation {
            root_id: 10,
            variation_id: climb.id,
        })
        .execute(conn);

    assert!(result.is_err());
}

/// Tests the variation_id foreign key constraint
#[test]
fn variation_fk() {
    let mut db = TestDatabase::with_migrations("test__climb_variation__variation_fk");
    let conn = db.connection();

    use climb_db::models::{Climb, NewClimb};
    use climb_db::schema::climbs;

    let climb = diesel::insert_into(climbs::table)
        .values(NewClimb {
            names: vec![Some("The Cheat".to_string())],
        })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::models::NewClimbVariation;
    use climb_db::schema::climb_variations;

    let result = diesel::insert_into(climb_variations::table)
        .values(NewClimbVariation {
            root_id: climb.id,
            variation_id: 10,
        })
        .execute(conn);

    assert!(result.is_err());
}

/// Tests the delete-cascade on root_id
#[test]
fn root_cascade() {
    let mut db = TestDatabase::with_migrations("test__climb_variation__root_cascade");
    let conn = db.connection();

    use climb_db::models::{Climb, NewClimb};
    use climb_db::schema::climbs;

    let root = diesel::insert_into(climbs::table)
        .values(NewClimb {
            names: vec![Some("The Cheat".to_string())],
        })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    let variation = diesel::insert_into(climbs::table)
        .values(NewClimb {
            names: vec![Some("The Chester".to_string())],
        })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::models::{NewClimbVariation,ClimbVariation};
    use climb_db::schema::climb_variations;

    let relation = diesel::insert_into(climb_variations::table)
        .values(NewClimbVariation {
            root_id: root.id,
            variation_id: variation.id,
        })
        .returning(ClimbVariation::as_returning())
        .get_result(conn)
        .expect("Failed to setup variation");

    diesel::delete(climbs::table)
        .filter(climbs::id.eq(root.id))
        .execute(conn)
        .expect("Failed to delete root");

    let result = climb_variations::table
        .find((relation.root_id, relation.variation_id))
        .first::<ClimbVariation>(conn);

    assert!(result.is_err())
}

/// Tests the delete-cascade on variation_id
#[test]
fn variation_cascade() {
    let mut db = TestDatabase::with_migrations("test__climb_variation__variation_cascade");
    let conn = db.connection();

    use climb_db::models::{Climb, NewClimb};
    use climb_db::schema::climbs;

    let root = diesel::insert_into(climbs::table)
        .values(NewClimb {
            names: vec![Some("The Cheat".to_string())],
        })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    let variation = diesel::insert_into(climbs::table)
        .values(NewClimb {
            names: vec![Some("The Chester".to_string())],
        })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::models::{NewClimbVariation,ClimbVariation};
    use climb_db::schema::climb_variations;

    let relation = diesel::insert_into(climb_variations::table)
        .values(NewClimbVariation {
            root_id: root.id,
            variation_id: variation.id,
        })
        .returning(ClimbVariation::as_returning())
        .get_result(conn)
        .expect("Failed to setup variation");

    diesel::delete(climbs::table)
        .filter(climbs::id.eq(variation.id))
        .execute(conn)
        .expect("Failed to delete variation");

    let result = climb_variations::table
        .find((relation.root_id, relation.variation_id))
        .first::<ClimbVariation>(conn);

    assert!(result.is_err())
}
