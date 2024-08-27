use climb_db::models::{Area, AreaBelongsTo, NewArea, NewAreaBelongsTo};
use diesel::prelude::*;
use common::TestDatabase;
use diesel::RunQueryDsl;
use diesel_migrations::MigrationHarness;

mod common;

/// Ensures a cascade delete constraint exists between areas.id and
/// area_belongs_to.area_id.
#[test]
pub fn cascade() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::new("test__area_belongs_to__cascade");

    let conn = db.connection();
    conn.run_pending_migrations(climb_db::MIGRATIONS).expect("Failed to initialize database");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("The Gallery".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    let super_area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("Old Baldy Mountain".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    use climb_db::schema::area_belongs_to;
    let relation = diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: area.id,
            super_area_id: super_area.id,
        })
        .returning(AreaBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    use climb_db::schema::areas::dsl::{ areas as areas_dsl, id as area_id };

    let num_deleted = diesel::delete(areas_dsl.filter(area_id.eq(area.id)))
        .execute(conn);

    assert_eq!(num_deleted, Ok(1));

    use climb_db::schema::area_belongs_to::dsl::area_belongs_to as abt_table;

    let result = abt_table.find(relation.area_id).first::<AreaBelongsTo>(conn).optional().expect("Failed");

    assert!(result.is_none());
}

/// Tests the foreign key constraints on the table
#[test]
pub fn foreign_key() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::new("test__area_belongs_to__foreign_key");

    let conn = db.connection();
    conn.run_pending_migrations(climb_db::MIGRATIONS).expect("Failed to initialize database");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("The Gallery".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    use climb_db::schema::area_belongs_to;
    let relation = diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: 10,
            super_area_id: area.id,
        })
        .returning(AreaBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());

    let relation = diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: area.id,
            super_area_id: 10,
        })
        .returning(AreaBelongsTo::as_returning())
        .get_result(conn);

    assert!(relation.is_err());
}

/// Tests the restrictions on the table
#[test]
pub fn restrict() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::new("test__area_belongs_to__restrict");

    let conn = db.connection();
    conn.run_pending_migrations(climb_db::MIGRATIONS).expect("Failed to initialize database");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("The Gallery".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    let super_area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("Old Baldy Mountain".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    use climb_db::schema::area_belongs_to;
    diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: area.id,
            super_area_id: super_area.id,
        })
        .returning(AreaBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    use climb_db::schema::areas::dsl::{ areas as areas_dsl, id as area_id };

    let result = diesel::delete(areas_dsl.filter(area_id.eq(super_area.id)))
        .execute(conn);

    assert!(result.is_err());
}

/// Ensures area cannot belong to itself. This is just an edge case of the no-cycles check.
#[test]
pub fn no_self_parent() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::new("test__area_belongs_to__no_self_parent");

    let conn = db.connection();
    conn.run_pending_migrations(climb_db::MIGRATIONS).expect("Failed to initialize database");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("The Gallery".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    use climb_db::schema::area_belongs_to;
    let result = diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: area.id,
            super_area_id: area.id,
        })
        .returning(AreaBelongsTo::as_returning())
        .get_result(conn);

    assert!(result.is_err());
}

/// Ensures cycles cannot be made with area_id and super_area_id.
#[test]
pub fn no_cycles() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::new("test__area_belongs_to__no_cycles");

    let conn = db.connection();
    conn.run_pending_migrations(climb_db::MIGRATIONS).expect("Failed to initialize database");

    use climb_db::schema::areas;

    let area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("The Gallery".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    let super_area = diesel::insert_into(areas::table)
        .values(NewArea { names: vec![Some("Old Baldy Mountain".to_string())] })
        .returning(Area::as_returning())
        .get_result(conn)
        .expect("Failed to insert area");

    use climb_db::schema::area_belongs_to;
    diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: area.id,
            super_area_id: super_area.id,
        })
        .returning(AreaBelongsTo::as_returning())
        .get_result(conn)
        .expect("Failed to insert relation");

    let result = diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: super_area.id,
            super_area_id: area.id,
        })
        .returning(AreaBelongsTo::as_returning())
        .get_result(conn);

    assert!(result.is_err());
}

