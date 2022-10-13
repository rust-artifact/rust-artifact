#[derive(Queryable)]
#[diesel(primary_key(address))]
pub struct Address {
    pub address: String,
    pub flags: u32,
}

bitflags! {
    #[derive(Default)]
    struct Flags: u32 {
        const LOCKED = 0b00000001;
        const MEMOFIELD = 0b00000010;
    }
}
