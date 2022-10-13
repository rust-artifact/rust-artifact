#[derive(Queryable)]
#[diesel(belongs_to(Address, foreign_key = address))]
#[diesel(belongs_to(Token, foreign_key = token))]
pub struct Balance {
    pub address: String,
    pub token: String,
    pub quantity: u64,
}
