use artifact::indexer::{Indexer};

fn main() {
    let indexer = Indexer::new().expect("Failed to create Indexer");

    match indexer.log_blocks_and_txids() {
        Ok(_) => println!("Finished logging blocks and transaction IDs."),
        Err(e) => eprintln!("Error while logging blocks and transaction IDs: {:?}", e),
    }
}
