use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::climbs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Climb {
    pub id: i32,
    pub names: Vec<Option<String>>,
}
