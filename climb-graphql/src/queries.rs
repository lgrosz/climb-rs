use std::str::FromStr;

use climbing_grades::verm;
use diesel::upsert::excluded;
use diesel::PgConnection;
use diesel::prelude::*;

use crate::schema::{Grade, GradeType, KVPair};

pub fn set_area_names(conn: &mut PgConnection, id: i32, names: Vec<String>) -> Result<(), String> {
    use climb_db::schema::areas;

    diesel::update(areas::table)
        .filter(areas::id.eq(id))
        .set(areas::names.eq(names))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_area_super_area_id(
    conn: &mut PgConnection,
    id: i32,
    super_area_id: i32,
) -> Result<(), String> {
    use climb_db::models::NewAreaBelongsTo;
    use climb_db::schema::area_belongs_to;

    diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: id,
            super_area_id,
        })
        .on_conflict(area_belongs_to::area_id)
        .do_update()
        .set(area_belongs_to::super_area_id.eq(excluded(area_belongs_to::super_area_id)))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_formation_area_id(conn: &mut PgConnection, id: i32, area_id: i32) -> Result<(), String> {
    use climb_db::schema::formation_belongs_to;
    use climb_db::models::NewFormationBelongsTo;

    diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: id,
            area_id: Some(area_id),
            super_formation_id: None,
        })
        .on_conflict(formation_belongs_to::formation_id)
        .do_update()
        .set((
            formation_belongs_to::area_id.eq(excluded(formation_belongs_to::area_id)),
            formation_belongs_to::super_formation_id.eq(None::<i32>),
        ))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_formation_super_formation_id(
    conn: &mut PgConnection,
    id: i32,
    super_formation_id: i32,
) -> Result<(), String> {
    use climb_db::models::NewFormationBelongsTo;
    use climb_db::schema::formation_belongs_to;

    diesel::insert_into(formation_belongs_to::table)
        .values(NewFormationBelongsTo {
            formation_id: id,
            area_id: None,
            super_formation_id: Some(super_formation_id),
        })
        .on_conflict(formation_belongs_to::formation_id)
        .do_update()
        .set((
            formation_belongs_to::area_id.eq(None::<i32>),
            formation_belongs_to::super_formation_id
                .eq(excluded(formation_belongs_to::super_formation_id)),
        ))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_climb_descriptions(
    conn: &mut PgConnection,
    id: i32,
    descriptions: Vec<KVPair>,
) -> Result<(), String> {
    use climb_db::schema::climb_description_types;

    // Map keys to climb_description_type_id
    let description_keys: Vec<String> = descriptions.iter().map(|kv| kv.key.clone()).collect();
    let type_ids_map: std::collections::HashMap<String, i32> = climb_description_types::table
        .filter(climb_description_types::name.eq_any(description_keys))
        .select((climb_description_types::name, climb_description_types::id))
        .load::<(String, i32)>(conn)
        .map_err(|e| e.to_string())?
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();

    use climb_db::models::NewClimbDescription;
    use climb_db::schema::climb_descriptions;

    // Insert descriptions
    let new_descriptions: Vec<NewClimbDescription> = descriptions
        .into_iter()
        .filter_map(|kv| {
            type_ids_map
                .get(&kv.key)
                .map(|type_id| NewClimbDescription {
                    climb_id: id,
                    climb_description_type_id: *type_id,
                    value: kv.value,
                })
        })
        .collect();

    diesel::insert_into(climb_descriptions::table)
        .values(&new_descriptions)
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_climb_grades(
    conn: &mut PgConnection,
    id: i32,
    grades: Vec<Grade>,
) -> Result<(), String> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let vermin_grades: Vec<u8> = grades
            .into_iter()
            .filter_map(|grade| {
                if grade.grade_type == GradeType::Vermin {
                    verm::Grade::from_str(grade.value.as_str())
                        .ok()
                        .map(|grade| grade.value())
                } else {
                    None
                }
            })
            .collect();

        use climb_db::schema::climb_vermin_grades;

        for value in vermin_grades {
            diesel::insert_into(climb_vermin_grades::table)
                .values((
                    climb_vermin_grades::climb_id.eq(id),
                    climb_vermin_grades::value.eq(value as i32),
                ))
                .on_conflict((climb_vermin_grades::climb_id, climb_vermin_grades::value))
                .do_nothing()
                .execute(conn)?;
        }

        Ok(())
    })
    .map_err(|e| e.to_string())
}

pub fn set_climb_area_id(
    conn: &mut PgConnection,
    id: i32,
    area_id: i32,
) -> Result<(), String> {
    use climb_db::models::NewClimbBelongsTo;
    use climb_db::schema::climb_belongs_to;

    diesel::insert_into(climb_belongs_to::table)
        .values(NewClimbBelongsTo {
            climb_id: id,
            area_id: Some(area_id),
            formation_id: None,
        })
        .on_conflict(climb_belongs_to::climb_id)
        .do_update()
        .set((
            climb_belongs_to::area_id.eq(excluded(climb_belongs_to::area_id)),
            climb_belongs_to::formation_id.eq(None::<i32>),
        ))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_climb_formation_id(
    conn: &mut PgConnection,
    id: i32,
    formation_id: i32,
) -> Result<(), String> {
    use climb_db::models::NewClimbBelongsTo;
    use climb_db::schema::climb_belongs_to;

    diesel::insert_into(climb_belongs_to::table)
        .values(NewClimbBelongsTo {
            climb_id: id,
            area_id: None,
            formation_id: Some(formation_id),
        })
        .on_conflict(climb_belongs_to::climb_id)
        .do_update()
        .set((
            climb_belongs_to::area_id.eq(None::<i32>),
            climb_belongs_to::formation_id.eq(excluded(climb_belongs_to::formation_id)),
        ))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
