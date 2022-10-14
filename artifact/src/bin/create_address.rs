use artifact::*;
use artifact::models::address::*;
use std::io::{stdin};

fn main() {
    let connection = &mut establish_connection();

    let mut address = String::new();

    println!("What would you like your address to be?");
    stdin().read_line(&mut address).unwrap();
    let address = address.trim_end();

    create_address(connection, &address, &0);
    println!("\nSaved address {}", address);
}
