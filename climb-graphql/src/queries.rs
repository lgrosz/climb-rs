use diesel::upsert::excluded;
use diesel::PgConnection;
use diesel::prelude::*;

pub fn set_area_names(conn: &mut PgConnection, id: i32, names: Vec<String>) -> Result<(), String> {
    use climb_db::schema::areas;

    diesel::update(areas::table)
        .filter(areas::id.eq(id))
        .set(areas::names.eq(names))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn set_area_super_area_id(conn: &mut PgConnection, id: i32, super_area_id: i32) -> Result<(), String> {
    use climb_db::schema::area_belongs_to;
    use climb_db::models::NewAreaBelongsTo;

    diesel::insert_into(area_belongs_to::table)
        .values(NewAreaBelongsTo {
            area_id: id,
            super_area_id
        })
        .on_conflict(area_belongs_to::area_id)
        .do_update()
        .set(area_belongs_to::super_area_id.eq(excluded(area_belongs_to::super_area_id)))
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
