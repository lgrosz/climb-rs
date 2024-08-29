use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::areas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Area {
    pub id: i32,
    pub names: Vec<Option<String>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::areas)]
pub struct NewArea {
    pub names: Vec<Option<String>>,
}

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

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::climb_belongs_to)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClimbBelongsTo {
    pub climb_id: i32,
    pub area_id: Option<i32>,
    pub formation_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::climb_belongs_to)]
pub struct NewClimbBelongsTo {
    pub climb_id: i32,
    pub area_id: Option<i32>,
    pub formation_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::formation_belongs_to)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FormationBelongsTo {
    pub formation_id: i32,
    pub area_id: Option<i32>,
    pub super_formation_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::formation_belongs_to)]
pub struct NewFormationBelongsTo {
    pub formation_id: i32,
    pub area_id: Option<i32>,
    pub super_formation_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::area_belongs_to)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AreaBelongsTo {
    pub area_id: i32,
    pub super_area_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::area_belongs_to)]
pub struct NewAreaBelongsTo {
    pub area_id: i32,
    pub super_area_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::grade_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GradeType {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::grade_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGradeType {
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::grades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Grade {
    pub id: i32,
    pub grade_type_id: i32,
    pub value: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::grades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGrade {
    pub grade_type_id: i32,
    pub value: String,
}

#[derive(Queryable)]
#[diesel(table_name = crate::schema::climb_grades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClimbGrade {
    pub climb_id: i32,
    pub grade_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::climb_grades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewClimbGrade {
    pub climb_id: i32,
    pub grade_id: i32,
}
