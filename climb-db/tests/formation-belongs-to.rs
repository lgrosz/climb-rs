use climb_db::models::{NewFormation, Formation, NewArea, Area, NewFormationBelongsTo, FormationBelongsTo};
use diesel::prelude::*;
use common::TestDatabase;
use diesel::RunQueryDsl;

mod common;

/// Ensures a cascade delete constraint exists between formations.id and
/// formation_belongs_to.area_id.
#[test]
pub fn cascade_area() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_belongs_to__cascade_area");
    let conn = db.connection();

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("Irie Heights Boulder".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("Irie Heights".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::formation_belongs_to;
    let relation = diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: Some(area.id),
            super_formation_id: None,
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    use climb_db::schema::formations::dsl::{ formations as formations_dsl, id as formation_id };

    let num_deleted = diesel::delete(formations_dsl.filter(formation_id.eq(formation.id)))
        .execute(conn);

    assert_eq!(num_deleted, Ok(1));

    use climb_db::schema::formation_belongs_to::dsl::formation_belongs_to as cbt_table;

    let result = cbt_table.find(relation.formation_id).first::<FormationBelongsTo>(conn).optional().expect("Failed");

    assert!(result.is_none());
}

/// Ensures a cascade delete constraint exists between formations.id and
/// formation_belongs_to.super_formation_id.
#[test]
pub fn cascade_super_formation() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_belongs_to__cascade_super_formation");
    let conn = db.connection();

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("The Diamond".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    let super_formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("Long's Peak".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::formation_belongs_to;
    let relation = diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: None,
            super_formation_id: Some(super_formation.id),
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    use climb_db::schema::formations::dsl::{ formations as formations_dsl, id as formation_id };

    let num_deleted = diesel::delete(formations_dsl.filter(formation_id.eq(formation.id)))
        .execute(conn);

    assert_eq!(num_deleted, Ok(1));

    use climb_db::schema::formation_belongs_to::dsl::formation_belongs_to as cbt_table;

    let result = cbt_table.find(relation.formation_id).first::<FormationBelongsTo>(conn).optional().expect("Failed");

    assert!(result.is_none());
}

/// Ensures a foreign key constraint exists between areas.id and formation_belongs_to.area_id, and
/// formations.id and formation_belongs_to.formation_id.
#[test]
pub fn foreign_key() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_belongs_to__foreign_key");
    let conn = db.connection();

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("Irie Heights Boulder".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::formation_belongs_to;
    let relation = diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: Some(1),
            super_formation_id: None,
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());

    let relation = diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: None,
            super_formation_id: Some(1),
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());
}

/// Ensures formation_belongs_to can relate formations to one of, but not both, areas and formations.
#[test]
pub fn one_parent() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_belongs_to__one_parent");
    let conn = db.connection();

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("The Diamond".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    let super_formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("Long's Peak".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("Rock Mountain National Park".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    use climb_db::schema::formation_belongs_to;
    let relation = diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: Some(area.id),
            super_formation_id: Some(super_formation.id)
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());

    let relation = diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: None,
            super_formation_id: None,
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());
}

/// Ensures there's a rectriction on deletion of areas if they relate a formation in
/// formation_belongs_to.
#[test]
pub fn restrict_area() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_belongs_to__restrict_area");
    let conn = db.connection();

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("Irie Heights Boulder".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("Irie Heights".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::formation_belongs_to;
    diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: Some(area.id),
            super_formation_id: None,
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    use climb_db::schema::areas::dsl::{ areas as areas_dsl, id as area_id };

    let result = diesel::delete(areas_dsl.filter(area_id.eq(area.id)))
        .execute(conn);

    assert!(result.is_err());
}

/// Ensures there's a rectriction on deletion of formations if they relate a formation in
/// formation_belongs_to.
#[test]
pub fn restrict_super_formation() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_belongs_to__restrict_super_formation");
    let conn = db.connection();

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("The Diamond".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    let super_formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("Long's Peak".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::formation_belongs_to;
    diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: None,
            super_formation_id: Some(super_formation.id),
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    use climb_db::schema::formations::dsl::{ formations as formations_dsl, id as formation_id };

    let result = diesel::delete(formations_dsl.filter(formation_id.eq(super_formation.id)))
        .execute(conn);

    assert!(result.is_err());
}

/// Ensures formation cannot belong to itself. This is just an edge case of the no-cycles check.
#[test]
pub fn no_self_parent() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_belongs_to__no_self_parent");
    let conn = db.connection();

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("The Diamond".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::formation_belongs_to;
    let result = diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: None,
            super_formation_id: Some(formation.id),
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn);

    assert!(result.is_err());
}

/// Ensures cycles cannot be made with formation_id and super_formation_id.
#[test]
pub fn no_cycles() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_belongs_to__no_cycles");
    let conn = db.connection();

    use climb_db::schema::formations;

    let formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("The Diamond".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    let super_formation = diesel::insert_into(formations::table)
        .values(NewFormation { names: vec![Some("Long's Peak".to_string())] })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    use climb_db::schema::formation_belongs_to;
    diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: formation.id,
            area_id: None,
            super_formation_id: Some(super_formation.id),
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    let result = diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: super_formation.id,
            area_id: None,
            super_formation_id: Some(formation.id)
        })
        .returning(FormationBelongsTo::as_returning())
        .get_result(conn);

    assert!(result.is_err());
}

