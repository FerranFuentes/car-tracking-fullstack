// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        matricula -> Varchar,
        hash -> Varchar,
        total_km -> Integer,
        date_created -> Timestamp,
    }
}
