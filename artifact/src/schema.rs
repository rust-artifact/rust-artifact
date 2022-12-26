// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (address) {
        address -> Text,
        flags -> Integer,
    }
}

diesel::table! {
    balances (address, token) {
        address -> Text,
        token -> Text,
        quantity -> Integer,
    }
}

diesel::table! {
    credits (address, token) {
        address -> Text,
        token -> Text,
        quantity -> Integer,
    }
}

diesel::table! {
    debits (address, token) {
        address -> Text,
        token -> Text,
        quantity -> Integer,
    }
}

diesel::table! {
    tokens (token) {
        token -> Text,
        flags -> Integer,
    }
}

diesel::joinable!(balances -> addresses (address));
diesel::joinable!(balances -> tokens (token));
diesel::joinable!(credits -> addresses (address));
diesel::joinable!(credits -> tokens (token));
diesel::joinable!(debits -> addresses (address));
diesel::joinable!(debits -> tokens (token));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    balances,
    credits,
    debits,
    tokens,
);
