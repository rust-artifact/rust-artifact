use artifact::*;
use artifact::models::address::*;
use std::io::{stdin};

fn main() {
    let connection = &mut establish_connection();

    let mut address = String::new();

    println!("What address would you like to update?");
    stdin().read_line(&mut address).unwrap();
    let address = address.trim_end();

    let mut flags = String::new();

    println!("What flags would you like to set?");
    stdin().read_line(&mut flags).unwrap();
    let flags: i32 = flags.trim_end().parse().unwrap();

    update_address(connection, address, &flags);
    println!("\nUpdated address {}", address);
}
