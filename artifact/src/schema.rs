// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (address) {
        address -> Text,
        flags -> Integer,
    }
}

diesel::table! {
    tokens (token) {
        token -> Text,
        flags -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    tokens,
);
