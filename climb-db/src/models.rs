use chrono::NaiveDate;
use std::ops::Bound;

use diesel::prelude::*;
use postgis_diesel::types::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::areas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Area {
    pub id: i32,
    pub names: Vec<Option<String>>,
}

#[derive(Insertable, Default)]
#[diesel(table_name = crate::schema::areas)]
pub struct NewArea {
    pub names: Vec<Option<String>>,
}

#[derive(Queryable)]
#[diesel(table_name = crate::schema::ascent_grades)]
#[diesel(primary_key(ascent_id, grade_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AscentGrade {
    pub ascent_id: i32,
    pub grade_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ascent_grades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAscentGrade {
    pub ascent_id: i32,
    pub grade_id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = crate::schema::ascent_parties)]
#[diesel(primary_key(ascent_id, climber_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AscentParty {
    pub ascent_id: i32,
    pub climber_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ascent_parties)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAscentParty {
    pub ascent_id: i32,
    pub climber_id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = crate::schema::ascents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ascent {
    pub id: i32,
    pub climb_id: i32,
    pub ascent_date: Option<(Bound<NaiveDate>, Bound<NaiveDate>)>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ascents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAscent {
    pub climb_id: i32,
    pub ascent_date: Option<(Bound<NaiveDate>, Bound<NaiveDate>)>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::climbers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Climber {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::climbers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewClimber {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::climbs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Climb {
    pub id: i32,
    pub names: Vec<Option<String>>,
}

#[derive(Insertable, Default)]
#[diesel(table_name = crate::schema::climbs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewClimb {
    pub names: Vec<Option<String>>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::formations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Formation {
    pub id: i32,
    pub names: Vec<Option<String>>,
    pub location: Option<Point>,
}

#[derive(Insertable, Default)]
#[diesel(table_name = crate::schema::formations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFormation {
    pub names: Vec<Option<String>>,
    pub location: Option<Point>,
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
#[diesel(check_for_backend(diesel::pg::Pg))]
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
#[diesel(check_for_backend(diesel::pg::Pg))]
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
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[derive(Insertable)]
#[diesel(table_name = crate::schema::climb_variations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewClimbVariation {
    pub root_id: i32,
    pub variation_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::climb_variations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClimbVariation {
    pub root_id: i32,
    pub variation_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::climb_description_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewClimbDescriptionType {
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::climb_description_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClimbDescriptionType {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::climb_descriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewClimbDescription {
    pub climb_id: i32,
    pub climb_description_type_id: i32,
    pub value: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::climb_descriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClimbDescription {
    pub climb_id: i32,
    pub climb_description_type_id: i32,
    pub value: String,
}
