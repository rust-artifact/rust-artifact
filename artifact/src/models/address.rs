use crate::schema::addresses;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(primary_key(address))]
pub struct Address {
    pub address: String,
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
    struct Flags: u32 {
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
