// @generated automatically by Diesel CLI.

diesel::table! {
    area_belongs_to (area_id) {
        area_id -> Int4,
        super_area_id -> Int4,
    }
}

diesel::table! {
    areas (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    climb_belongs_to (climb_id) {
        climb_id -> Int4,
        area_id -> Nullable<Int4>,
        formation_id -> Nullable<Int4>,
    }
}

diesel::table! {
    climb_grades (climb_id, grade_id) {
        climb_id -> Int4,
        grade_id -> Int4,
    }
}

diesel::table! {
    climbs (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    formation_belongs_to (formation_id) {
        formation_id -> Int4,
        area_id -> Nullable<Int4>,
        super_formation_id -> Nullable<Int4>,
    }
}

diesel::table! {
    formations (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    grade_types (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    grades (id) {
        id -> Int4,
        grade_type_id -> Int4,
        #[max_length = 50]
        value -> Varchar,
    }
}

diesel::joinable!(climb_belongs_to -> areas (area_id));
diesel::joinable!(climb_belongs_to -> climbs (climb_id));
diesel::joinable!(climb_belongs_to -> formations (formation_id));
diesel::joinable!(climb_grades -> climbs (climb_id));
diesel::joinable!(climb_grades -> grades (grade_id));
diesel::joinable!(formation_belongs_to -> areas (area_id));
diesel::joinable!(grades -> grade_types (grade_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    area_belongs_to,
    areas,
    climb_belongs_to,
    climb_grades,
    climbs,
    formation_belongs_to,
    formations,
    grade_types,
    grades,
);
