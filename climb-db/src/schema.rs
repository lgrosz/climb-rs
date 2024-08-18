// @generated automatically by Diesel CLI.

diesel::table! {
    areas (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    climbs (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    formations (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    areas,
    climbs,
    formations,
);
