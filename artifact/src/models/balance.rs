use crate::schema::balances;
use diesel::prelude::*;
use validator::Validate;

#[derive(Queryable, Validate)]
#[diesel(belongs_to(Address, foreign_key = address))]
#[diesel(belongs_to(Token, foreign_key = token))]
pub struct Balance {
    pub address: String,
    pub token: String,
    #[validate(range(min = 0, max = 10000))]
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = balances)]
pub struct NewBalance<'a> {
    pub address: &'a str,
    pub token: &'a str,
    pub quantity: &'a i32,
}

/// Save to DB
pub fn create_balance(conn: &mut SqliteConnection, address: &str, token: &str, quantity: &i32) {
    let new_balance = NewBalance {
        address,
        token,
        quantity,
    };

    diesel::insert_into(balances::table)
        .values(&new_balance)
        .execute(conn)
        .expect("Error saving new balance");
}

/// Save to DB
pub fn update_balance(conn: &mut SqliteConnection, address: &str, token: &str) {
    use crate::schema::credits::dsl::credits;
    use crate::schema::debits::dsl::debits;
    use crate::schema::balances::dsl::balances;
    use diesel::dsl::sum;

    let quantity = credits.select(sum(credits::quantity))
                .filter(credits::address.eq(address))
                .filter(credits::token.eq(token))
                .single_value()
        - debits.select(sum(debits::quantity))
                .filter(debits::address.eq(address))
                .filter(debits::token.eq(token))
                .single_value();

    diesel::update(balances.find((address, token)))
        .set(balances::quantity.eq(quantity))
        .execute(conn)
        .expect("Error updating balance");
}
