use crate::schema::credits;
use diesel::prelude::*;
use validator::Validate;

#[derive(Queryable, Validate)]
#[diesel(belongs_to(Address, foreign_key = address))]
#[diesel(belongs_to(Token, foreign_key = token))]
pub struct Credit {
    pub address: String,
    pub token: String,
    #[validate(range(min = 0, max = 10000))]
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = credits)]
pub struct NewCredit<'a> {
    pub address: &'a str,
    pub token: &'a str,
    pub quantity: &'a i32,
}

/// Save to DB
pub fn create_credit(conn: &mut SqliteConnection, address: &str, token: &str, quantity: &i32) {
    let new_credit = NewCredit {
        address,
        token,
        quantity,
    };

    diesel::insert_into(credits::table)
        .values(&new_credit)
        .execute(conn)
        .expect("Error saving new credit");
}
