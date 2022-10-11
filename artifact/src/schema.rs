// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (name) {
        name -> Text,
        flags -> Integer,
    }
}
