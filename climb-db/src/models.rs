use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::climbs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Climb {
    pub id: i32,
    pub names: Vec<Option<String>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::climbs)]
pub struct NewClimb {
    pub names: Vec<Option<String>>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::formations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Formation {
    pub id: i32,
    pub names: Vec<Option<String>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::formations)]
pub struct NewFormation {
    pub names: Vec<Option<String>>,
}
