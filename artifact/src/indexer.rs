use crate::options::BitcoinRpcOptions;
use bitcoincore_rpc::{Client, RpcApi};
use bitcoin::Block;
use std::thread;
use std::time::Duration;

pub struct BlockData {
    pub block: Block,
}

pub struct Indexer {
    rpc_client: Client,
}

impl Indexer {
    pub fn new() -> Result<Self, bitcoincore_rpc::Error> {
        let options = BitcoinRpcOptions::new();
        let rpc_client = options.create_rpc_client()?;
        Ok(Self { rpc_client })
    }

    pub fn fetch_latest_block(&self) -> bitcoincore_rpc::Result<BlockData> {
        let best_block_hash = self.rpc_client.get_best_block_hash()?;
        let block = self.rpc_client.get_block(&best_block_hash)?;

        Ok(BlockData { block })
    }

    pub fn log_blocks_and_txids(&self) -> Result<(), String> {
        let mut height = 0;
        let mut retries = 0;

        loop {
            match self.fetch_block_with_retries(height) {
                Ok(Some(block)) => {
                    println!("Block number: {}", height);
                    for txid in block.txdata.iter().map(|tx| tx.txid()) {
                        println!("TxID: {}", txid);
                    }
                    height += 1;
                    retries = 0; // Reset retry count after a successful fetch
                }
                Ok(None) => break, // No more blocks to fetch
                Err(e) => {
                    retries += 1;
                    if retries > 5 {
                        return Err(format!("Error: Too many retries, aborting. Last error: {:?}", e));
                    }
                    let wait_time = 2u64.pow(retries);
                    eprintln!("Retry {retries}: waiting {wait_time} seconds...");
                    thread::sleep(Duration::from_secs(wait_time));
                }
            }
        }

        Ok(())
    }

    fn fetch_block_with_retries(&self, height: u32) -> Result<Option<Block>, String> {
        match self.rpc_client.get_block_hash(height.into()) {
            Ok(hash) => {
                match self.rpc_client.get_block(&hash) {
                    Ok(block) => Ok(Some(block)),
                    Err(e) => {
                        eprintln!("Error fetching block: {:?}", e);
                        Ok(None)
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching block hash: {:?}", e);
                Err(e.to_string())
            }
        }
    }
}
