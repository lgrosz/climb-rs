// @generated automatically by Diesel CLI.

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

diesel::joinable!(climb_belongs_to -> areas (area_id));
diesel::joinable!(climb_belongs_to -> climbs (climb_id));
diesel::joinable!(climb_belongs_to -> formations (formation_id));
diesel::joinable!(formation_belongs_to -> areas (area_id));

diesel::allow_tables_to_appear_in_same_query!(
    areas,
    climb_belongs_to,
    climbs,
    formation_belongs_to,
    formations,
);
