use artifact::models::balance::*;
use artifact::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut address = String::new();

    println!("What address would you like to update?");
    stdin().read_line(&mut address).unwrap();
    let address = address.trim_end();

    let mut token = String::new();

    println!("What token balance should be updated?");
    stdin().read_line(&mut token).unwrap();
    let token = token.trim_end().to_uppercase();

    update_balance(connection, address, &token);
    println!("\nSaved balance {}", address);
}
