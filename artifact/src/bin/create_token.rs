use artifact::*;
use artifact::models::token::*;
use std::io::{stdin};

fn main() {
    let connection = &mut establish_connection();

    let mut token = String::new();

    println!("What would you like your token to be?");
    stdin().read_line(&mut token).unwrap();
    let token = token.trim_end().to_uppercase();

    let flags = Flags::LOCKED | Flags::NAMESPACE;

    create_token(connection, &token, &flags.bits());
    println!("\nSaved token {}", token);
}
