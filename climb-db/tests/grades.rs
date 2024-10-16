use common::TestDatabase;
use diesel::prelude::*;

mod common;

/// Allows insertion of new, unique, grade type
#[test]
fn insert_grade_type() {
    let mut db = TestDatabase::with_migrations("test__grades__insert_grade_type");
    let conn = db.connection();

    use climb_db::schema::grade_types;
    use climb_db::models::{GradeType, NewGradeType};

    let name = "basecamp".to_string();

    let result = diesel::insert_into(grade_types::table)
        .values(NewGradeType { name: name.clone() })
        .returning(GradeType::as_returning())
        .get_result(conn);

    assert!(result.is_ok());

    let grade_type = result.unwrap();
    assert_eq!(grade_type.name, name);
}

/// Expect default grade types to exist
#[test]
fn default_grade_types() {
    let mut db = TestDatabase::with_migrations("test__grades__default_grade_types");
    let conn = db.connection();

    let default_grade_names = [
        "vermin".to_string(),
    ];

    use climb_db::models::GradeType;
    use climb_db::schema::grade_types;

    // Create a query to select all rows
    let grade_types = grade_types::table
        .load::<GradeType>(conn)
        .expect("Error loading rows");

    let mut expected_grade_names: Vec<String> = default_grade_names.to_vec();
    let mut grade_names: Vec<String> = grade_types.iter().map(|t| t.name.clone()).collect();

    grade_names.sort();
    expected_grade_names.sort();

    assert_eq!(grade_names, expected_grade_names);
}

/// Ensures grades get removed when removing their type
#[test]
fn grades_cascade_on_grade_type_delete() {
    let mut db = TestDatabase::with_migrations("test__grades__grades_cascade_on_grade_type_delete");
    let conn = db.connection();

    use climb_db::models::GradeType;
    use climb_db::schema::grade_types;

    let grade_type = grade_types::table
        .filter(grade_types::name.eq("vermin"))
        .limit(1)
        .select(GradeType::as_select())
        .first(conn)
        .expect("Could not get vermin grade type");

    use climb_db::models::NewGrade;
    use climb_db::schema::grades;

    let rows_inserted = diesel::insert_into(grades::table)
        .values(NewGrade { grade_type_id: grade_type.id, value: "V5".to_string() })
        .execute(conn);

    assert_eq!(Ok(1), rows_inserted);

    let delete_grade_type_result = diesel::delete(grade_types::table)
        .filter(grade_types::id.eq(grade_type.id))
        .execute(conn);

    assert!(delete_grade_type_result.is_ok());

    use diesel::dsl::count;

    let vermin_grade_count = grades::table
        .filter(grades::grade_type_id.eq(grade_type.id))
        .select(count(grades::grade_type_id))
        .first(conn);

    assert_eq!(Ok(0), vermin_grade_count);
}
