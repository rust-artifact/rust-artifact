use diesel::prelude::*;
use crate::schema::addresses;
use validator::{Validate};

#[derive(Queryable, Validate)]
#[diesel(primary_key(address))]
pub struct Address {
    pub address: String,
    #[validate(range(min = 0, max = 3))]
    pub flags: i32,
}

#[derive(Insertable)]
#[diesel(table_name = addresses)]
pub struct NewAddress<'a> {
    pub address: &'a str,
    pub flags: &'a i32,
}

bitflags! {
    #[derive(Default)]
    struct Flags: i32 {
        const LOCKED = 0b00000001;
        const MEMOFIELD = 0b00000010;
    }
}

/// Save to DB
pub fn create_address(conn: &mut SqliteConnection, address: &str, flags: &i32) {
    let new_address = NewAddress { address, flags };

    diesel::insert_into(addresses::table)
        .values(&new_address)
        .execute(conn)
        .expect("Error saving new address");
}

/// Save to DB
pub fn update_address(conn: &mut SqliteConnection, address: &str, new_flags: &i32) {
    use crate::models::address::addresses::dsl::addresses;
    use crate::models::address::addresses::dsl::flags;

    diesel::update(addresses.find(address))
        .set(flags.eq(new_flags))
        .execute(conn)
        .expect("Error updating address");
}
