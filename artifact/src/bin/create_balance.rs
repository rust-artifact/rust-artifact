use artifact::models::balance::*;
use artifact::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut address = String::new();

    println!("What would you like your address to be?");
    stdin().read_line(&mut address).unwrap();
    let address = address.trim_end();

    let mut token = String::new();

    println!("What would you like your token to be?");
    stdin().read_line(&mut token).unwrap();
    let token = token.trim_end().to_uppercase();

    create_balance(connection, address, &token, &1);
    println!("\nSaved balance {}", address);
}
