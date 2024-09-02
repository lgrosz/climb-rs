use diesel::prelude::*;
use common::TestDatabase;
use diesel::RunQueryDsl;
use postgis_diesel::types::Point;

mod common;

// Some helper functions
use postgis_diesel::sql_types::Geometry;
use diesel::sql_types::Nullable;

diesel::define_sql_function! {
    fn st_y(geom: Nullable<Geometry>) -> Nullable<Double>;
}


/// Tests the functionality of PostGIS doing a PostGIS-based query
#[test]
pub fn simple() {
    // TODO [TestName](https://doc.rust-lang.org/test/enum.TestName.html)
    let mut db = TestDatabase::with_migrations("test__formation_spacial__simple");
    let conn = db.connection();

    use climb_db::models::NewFormation;
    use climb_db::schema::formations;

    let hydra_boulder = diesel::insert_into(formations::table)
        .values(NewFormation {
            names: vec![Some("Hydra Boulder".to_string())],
            location: Some(Point { x: 43.889938, y: -103.456774, srid: Some(4326) }),
        })
        .returning(Formation::as_returning())
        .get_result(conn)
        .expect("Failed to insert formation");

    diesel::insert_into(formations::table)
        .values(NewFormation {
            names: vec![Some("Irie Heights Boulder".to_string())],
            location: Some(Point { x: 43.892805, y: -103.459500, srid: Some(4326) }),
        })
        .execute(conn)
        .expect("Failed to insert formation");

    let latitude = 43.890917;

    use climb_db::models::Formation;

    let result = formations::table
        .select(Formation::as_select())
        .filter(formations::location.is_not_null())
        .filter(st_y(formations::location).lt(latitude))
        .load::<Formation>(conn);

    assert!(result.is_ok());

    let southern_formations = result.unwrap();

    assert!(southern_formations.iter().any(|f| f.id == hydra_boulder.id));
}
