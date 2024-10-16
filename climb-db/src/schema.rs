// @generated automatically by Diesel CLI.

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    area_belongs_to (area_id) {
        area_id -> Int4,
        super_area_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    areas (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    ascent_grades (ascent_id, grade_id) {
        ascent_id -> Int4,
        grade_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    ascent_parties (ascent_id, climber_id) {
        ascent_id -> Int4,
        climber_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    ascents (id) {
        id -> Int4,
        climb_id -> Int4,
        ascent_date -> Nullable<Daterange>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    climb_belongs_to (climb_id) {
        climb_id -> Int4,
        area_id -> Nullable<Int4>,
        formation_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    climb_description_types (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    climb_descriptions (climb_id, climb_description_type_id) {
        climb_id -> Int4,
        climb_description_type_id -> Int4,
        value -> Text,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    climb_variations (root_id, variation_id) {
        root_id -> Int4,
        variation_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    climb_vermin_grades (climb_id, value) {
        climb_id -> Int4,
        value -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    climbers (id) {
        id -> Int4,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    climbs (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    formation_belongs_to (formation_id) {
        formation_id -> Int4,
        area_id -> Nullable<Int4>,
        super_formation_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    formations (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
        location -> Nullable<Geometry>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    grade_types (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    grades (id) {
        id -> Int4,
        grade_type_id -> Int4,
        #[max_length = 50]
        value -> Varchar,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::joinable!(ascent_grades -> ascents (ascent_id));
diesel::joinable!(ascent_grades -> grades (grade_id));
diesel::joinable!(ascents -> climbs (climb_id));
diesel::joinable!(climb_belongs_to -> areas (area_id));
diesel::joinable!(climb_belongs_to -> climbs (climb_id));
diesel::joinable!(climb_belongs_to -> formations (formation_id));
diesel::joinable!(climb_descriptions -> climb_description_types (climb_description_type_id));
diesel::joinable!(climb_descriptions -> climbs (climb_id));
diesel::joinable!(climb_vermin_grades -> climbs (climb_id));
diesel::joinable!(formation_belongs_to -> areas (area_id));
diesel::joinable!(grades -> grade_types (grade_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    area_belongs_to,
    areas,
    ascent_grades,
    ascent_parties,
    ascents,
    climb_belongs_to,
    climb_description_types,
    climb_descriptions,
    climb_variations,
    climb_vermin_grades,
    climbers,
    climbs,
    formation_belongs_to,
    formations,
    grade_types,
    grades,
    spatial_ref_sys,
);
