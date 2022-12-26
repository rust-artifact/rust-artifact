use artifact::models::token;

fn main() {
    println!("{}", token::generate_id("A"));
    println!("{}", token::generate_id("BTC"));
    println!("{}", token::generate_id("XCP"));
    println!("{}", token::generate_id("ZZZZ"));
    println!("{}", token::generate_id("ZZZZZZZZZZZZ"));
    println!("{}", token::generate_id("999999999999"));
    println!("{}", token::generate_token(2));
    println!("{}", token::generate_token(5134));
    println!("{}", token::generate_token(36269));
    println!("{}", token::generate_token(1521585));
    println!("{}", token::generate_token(6615538473766618305));
    println!("{}", token::generate_token(9065737908494995455));
}
