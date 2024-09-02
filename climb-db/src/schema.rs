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

    climb_grades (climb_id, grade_id) {
        climb_id -> Int4,
        grade_id -> Int4,
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

diesel::joinable!(climb_belongs_to -> areas (area_id));
diesel::joinable!(climb_belongs_to -> climbs (climb_id));
diesel::joinable!(climb_belongs_to -> formations (formation_id));
diesel::joinable!(climb_descriptions -> climb_description_types (climb_description_type_id));
diesel::joinable!(climb_descriptions -> climbs (climb_id));
diesel::joinable!(climb_grades -> climbs (climb_id));
diesel::joinable!(climb_grades -> grades (grade_id));
diesel::joinable!(formation_belongs_to -> areas (area_id));
diesel::joinable!(grades -> grade_types (grade_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    area_belongs_to,
    areas,
    climb_belongs_to,
    climb_description_types,
    climb_descriptions,
    climb_grades,
    climb_variations,
    climbs,
    formation_belongs_to,
    formations,
    grade_types,
    grades,
    spatial_ref_sys,
);
