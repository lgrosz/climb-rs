use common::TestDatabase;
use diesel::prelude::*;

mod common;

/// Tests the climb_id foreign key constraint
#[test]
fn climb_fk() {
    let mut db = TestDatabase::with_migrations("test__climb_descriptions__climb_fk");
    let conn = db.connection();

    use climb_db::schema::climb_description_types;
    use climb_db::models::NewClimbDescriptionType;
    use climb_db::models::ClimbDescriptionType;

    let brief_type = diesel::insert_into(climb_description_types::table)
        .values(NewClimbDescriptionType { name: "brief".to_string() })
        .on_conflict(climb_description_types::name)
        .do_update()
        .set(climb_description_types::name.eq(climb_description_types::name))
        .returning(ClimbDescriptionType::as_returning())
        .get_result(conn)
        .expect("Failed to upsert brief type");

    use climb_db::models::NewClimbDescription;
    use climb_db::schema::climb_descriptions;

    let result = diesel::insert_into(climb_descriptions::table)
        .values(NewClimbDescription {
            climb_description_type_id: brief_type.id,
            climb_id: 10,
            value: "SDS and follow incuts to a cruxy sequence to gain the lip".to_string(),
        })
        .execute(conn);

    assert!(result.is_err());
}

/// Tests the climb_description_type_id foreign key constraint
#[test]
fn climb_description_type_fk() {
    let mut db = TestDatabase::with_migrations("test__climb_descriptions__climb_description_type_fk");
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

    use climb_db::models::NewClimbDescription;
    use climb_db::schema::climb_descriptions;

    let result = diesel::insert_into(climb_descriptions::table)
        .values(NewClimbDescription {
            climb_description_type_id: 10,
            climb_id: climb.id,
            value: "SDS and follow incuts to a cruxy sequence to gain the lip".to_string(),
        })
        .execute(conn);

    assert!(result.is_err());
}

/// Tests the delete-cascade on climb_id
#[test]
fn climb_cascade() {
    let mut db = TestDatabase::with_migrations("test__climb_descriptions__climb_cascade");
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

    use climb_db::schema::climb_description_types;
    use climb_db::models::NewClimbDescriptionType;
    use climb_db::models::ClimbDescriptionType;

    let brief_type = diesel::insert_into(climb_description_types::table)
        .values(NewClimbDescriptionType { name: "brief".to_string() })
        .on_conflict(climb_description_types::name)
        .do_update()
        .set(climb_description_types::name.eq(climb_description_types::name))
        .returning(ClimbDescriptionType::as_returning())
        .get_result(conn)
        .expect("Failed to upsert brief type");

    use climb_db::models::{NewClimbDescription,ClimbDescription};
    use climb_db::schema::climb_descriptions;

    let description = diesel::insert_into(climb_descriptions::table)
        .values(NewClimbDescription {
            climb_description_type_id: brief_type.id,
            climb_id: climb.id,
            value: "SDS and follow incuts to a cruxy sequence to gain the lip".to_string(),
        })
        .returning(ClimbDescription::as_returning())
        .get_result(conn)
        .expect("Failed to add climb description");

    diesel::delete(climbs::table)
        .filter(climbs::id.eq(description.climb_id))
        .execute(conn)
        .expect("Failed to delete climb");

    let result = climb_descriptions::table
        .find((description.climb_id, description.climb_description_type_id))
        .first::<ClimbDescription>(conn)
        .optional()
        .expect("Failed");

    assert!(result.is_none());
}

/// Tests the delete-cascade on climb_description_type_id
#[test]
fn climb_description_type_cascade() {
    let mut db = TestDatabase::with_migrations("test__climb_descriptions__climb_description_type_cascade");
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

    use climb_db::schema::climb_description_types;
    use climb_db::models::NewClimbDescriptionType;
    use climb_db::models::ClimbDescriptionType;

    let brief_type = diesel::insert_into(climb_description_types::table)
        .values(NewClimbDescriptionType { name: "brief".to_string() })
        .on_conflict(climb_description_types::name)
        .do_update()
        .set(climb_description_types::name.eq(climb_description_types::name))
        .returning(ClimbDescriptionType::as_returning())
        .get_result(conn)
        .expect("Failed to upsert brief type");

    use climb_db::models::{NewClimbDescription,ClimbDescription};
    use climb_db::schema::climb_descriptions;

    let description = diesel::insert_into(climb_descriptions::table)
        .values(NewClimbDescription {
            climb_description_type_id: brief_type.id,
            climb_id: climb.id,
            value: "SDS and follow incuts to a cruxy sequence to gain the lip".to_string(),
        })
        .returning(ClimbDescription::as_returning())
        .get_result(conn)
        .expect("Failed to add climb description");

    diesel::delete(climb_description_types::table)
        .filter(climb_description_types::id.eq(description.climb_description_type_id))
        .execute(conn)
        .expect("Failed to delete description type");

    let result = climb_descriptions::table
        .find((description.climb_id, description.climb_description_type_id))
        .first::<ClimbDescription>(conn)
        .optional()
        .expect("Failed");

    assert!(result.is_none());
}

/// Expect default grade types to exist
#[test]
fn initial_description_types() {
    let mut db = TestDatabase::with_migrations("test__climb_descriptions__initial_description_types");
    let conn = db.connection();

    let default_desc_type_names = [
        "brief".to_string(),
        "desc".to_string(),
        "hist".to_string(),
    ];

    use climb_db::models::ClimbDescriptionType;
    use climb_db::schema::climb_description_types;

    // Create a query to select all rows
    let desc_types = climb_description_types::table
        .load::<ClimbDescriptionType>(conn)
        .expect("Error loading rows");

    let mut expected_desc_names: Vec<String> = default_desc_type_names.to_vec();
    let mut desc_names: Vec<String> = desc_types.iter().map(|t| t.name.clone()).collect();

    desc_names.sort();
    expected_desc_names.sort();

    assert_eq!(desc_names, expected_desc_names);
}
