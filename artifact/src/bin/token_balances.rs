use artifact::models::balance::*;
use artifact::schema::balances::dsl::*;
use artifact::*;
use diesel::prelude::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut query = String::new();

    println!("What token balances would you like?");
    stdin().read_line(&mut query).unwrap();
    let query = query.trim_end();

    let results = balances
        .filter(token.eq(query))
        .load::<Balance>(connection)
        .expect("Error loading balances");

    println!("Displaying {} balances", results.len());
    for balance in results {
        println!("{} {}", balance.address, balance.quantity);
    }
}
