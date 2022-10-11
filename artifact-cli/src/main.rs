use artifact::models::token;

fn main() {
    println!("{}", token::generate_id("A"));
    println!("{}", token::generate_id("BTC"));
    println!("{}", token::generate_id("XCP"));
    println!("{}", token::generate_id("ZZZZ"));
    println!("{}", token::generate_id("ZZZZZZZZZZZZZ"));
    println!("{}", token::generate_name(1));
    println!("{}", token::generate_name(2001));
    println!("{}", token::generate_name(4052555153018976266));
}
