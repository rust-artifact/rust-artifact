use artifact::models::token::*;
use artifact::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut token = String::new();

    println!("What token would you like to update?");
    stdin().read_line(&mut token).unwrap();
    let token = token.trim_end().to_uppercase();

    let mut flags = String::new();

    println!("What flags would you like to set?");
    stdin().read_line(&mut flags).unwrap();
    let flags: i32 = flags.trim_end().parse().unwrap();

    update_token(connection, &token, &flags);
    println!("\nUpdated token {}", token);
}
