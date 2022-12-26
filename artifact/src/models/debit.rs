use crate::schema::debits;
use diesel::prelude::*;
use validator::Validate;

#[derive(Queryable, Validate)]
#[diesel(belongs_to(Address, foreign_key = address))]
#[diesel(belongs_to(Token, foreign_key = token))]
pub struct Debit {
    pub address: String,
    pub token: String,
    #[validate(range(min = 0, max = 10000))]
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = debits)]
pub struct NewDebit<'a> {
    pub address: &'a str,
    pub token: &'a str,
    pub quantity: &'a i32,
}

/// Save to DB
pub fn create_debit(conn: &mut SqliteConnection, address: &str, token: &str, quantity: &i32) {
    let new_debit = NewDebit {
        address,
        token,
        quantity,
    };

    diesel::insert_into(debits::table)
        .values(&new_debit)
        .execute(conn)
        .expect("Error saving new debit");
}
