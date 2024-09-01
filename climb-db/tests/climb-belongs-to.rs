use climb_db::models::{NewClimb, Climb, NewArea, Area, NewFormation, Formation, NewClimbBelongsTo, ClimbBelongsTo};
use diesel::prelude::*;
use common::TestDatabase;
use diesel::RunQueryDsl;

mod common;

/// Ensures a cascade delete constraint exists between climbs.id and climb_belongs_to.climb_id.
#[test]
pub fn cascade() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__climb_belongs_to__cascade");
    let conn = db.connection();

    use climb_db::schema::climbs;

    let climb = diesel::insert_into(climbs::table)
        .values(NewClimb { names: vec![Some("The Cheat".to_string())] })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation {
            names: vec![Some("North Nostril Cave".to_string())],
            ..Default::default()
        })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::climb_belongs_to;
    let relation = diesel::insert_into(climb_belongs_to::table)
        .values(NewClimbBelongsTo {
            climb_id: climb.id,
            area_id: None,
            formation_id: Some(formation.id)
        })
        .returning(ClimbBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    use climb_db::schema::climbs::dsl::{ climbs as climbs_dsl, id as climb_id };

    let num_deleted = diesel::delete(climbs_dsl.filter(climb_id.eq(climb.id)))
        .execute(conn);

    assert_eq!(num_deleted, Ok(1));

    use climb_db::schema::climb_belongs_to::dsl::climb_belongs_to as cbt_table;

    let result = cbt_table.find(relation.climb_id).first::<ClimbBelongsTo>(conn).optional().expect("Failed");

    assert!(result.is_none());
}

/// Ensures a foreign key constraint exists between areas.id and climb_belongs_to.area_id, and
/// formations.id and climb_belongs_to.formation_id.
#[test]
pub fn foreign_key() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__climb_belongs_to__foreign_key");
    let conn = db.connection();

    use climb_db::schema::climbs;

    let climb = diesel::insert_into(climbs::table)
        .values(NewClimb { names: vec![Some("The Cheat".to_string())] })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::schema::climb_belongs_to;
    let relation = diesel::insert_into(climb_belongs_to::table)
        .values(NewClimbBelongsTo {
            climb_id: climb.id,
            area_id: Some(1),
            formation_id: None,
        })
        .returning(ClimbBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());

    let relation = diesel::insert_into(climb_belongs_to::table)
        .values(NewClimbBelongsTo {
            climb_id: climb.id,
            area_id: None,
            formation_id: Some(1),
        })
        .returning(ClimbBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());
}

/// Ensures climb_belongs_to can relate climbs to one of, but not both, areas and formations.
#[test]
pub fn one_parent() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__climb_belongs_to__one_parent");
    let conn = db.connection();

    use climb_db::schema::climbs;

    let climb = diesel::insert_into(climbs::table)
        .values(NewClimb { names: vec![Some("The Cheat".to_string())] })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation {
            names: vec![Some("North Nostril Cave".to_string())],
            ..Default::default()
        })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("North Nostril Cave".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    use climb_db::schema::climb_belongs_to;
    let relation = diesel::insert_into(climb_belongs_to::table)
        .values(NewClimbBelongsTo {
            climb_id: climb.id,
            area_id: Some(area.id),
            formation_id: Some(formation.id)
        })
        .returning(ClimbBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());

    let relation = diesel::insert_into(climb_belongs_to::table)
        .values(NewClimbBelongsTo {
            climb_id: climb.id,
            area_id: None,
            formation_id: None,
        })
        .returning(ClimbBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());
}

/// Ensures there's a rectriction on deletion of areas and formations if they relate a climb in
/// climb_belongs_to.
#[test]
pub fn restrict() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__climb_belongs_to__restrict");
    let conn = db.connection();

    use climb_db::schema::climbs;

    let climb = diesel::insert_into(climbs::table)
        .values(NewClimb { names: vec![Some("The Cheat".to_string())] })
        .returning(Climb::as_returning())
        .get_result(conn)
        .expect("Failed to insert climb");

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation {
            names: vec![Some("North Nostril Cave".to_string())],
            ..Default::default()
        })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::climb_belongs_to;
    diesel::insert_into(climb_belongs_to::table)
        .values(NewClimbBelongsTo {
            climb_id: climb.id,
            area_id: None,
            formation_id: Some(formation.id)
        })
        .returning(ClimbBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    use climb_db::schema::formations::dsl::{ formations as formations_dsl, id as formation_id };

    let result = diesel::delete(formations_dsl.filter(formation_id.eq(formation.id)))
        .execute(conn);

    assert!(result.is_err());
}

