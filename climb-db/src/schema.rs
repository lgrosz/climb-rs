// @generated automatically by Diesel CLI.

diesel::table! {
    climbs (id) {
        id -> Int4,
        names -> Array<Nullable<Text>>,
    }
}
