use artifact::*;
use artifact::models::balance::*;
use std::io::{stdin};

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

    let mut quantity = String::new();

    println!("What quantity would you like to set?");
    stdin().read_line(&mut quantity).unwrap();
    let quantity: i32 = quantity.trim_end().parse().unwrap();

    update_balance(connection, address, &token, &quantity);
    println!("\nSaved balance {}", address);
}
