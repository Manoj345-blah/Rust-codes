use crypto_hash::{hex_digest, Algorithm};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    timestamp: u64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(data: &str, prev_hash: &str) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_secs();
        let data = data.to_string();
        let mut combined_data = format!("{}{}{}", timestamp, data, prev_hash);
        let hash = hex_digest(Algorithm::SHA256, combined_data.as_bytes());
        combined_data.clear();

        Block {
            timestamp,
            data,
            prev_hash: prev_hash.to_string(),
            hash,
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new("Genesis block", "0");
        
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: &str) {
        let prev_hash = &self.blocks.last().unwrap().hash;
        let block = Block::new(data, prev_hash);
        self.blocks.push(block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1];

            if current_block.hash != hex_digest(Algorithm::SHA256, format!("{}{}{}", current_block.timestamp, current_block.data, current_block.prev_hash).as_bytes()) {
                return false;
            }

            if current_block.prev_hash != prev_block.hash {
                return false;
            }
        }

        true
    }
}

fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();

    // Add some blocks
    blockchain.add_block("First block");
    blockchain.add_block("Second block");

    // Check if blockchain is valid
    println!("Blockchain is valid: {}", blockchain.is_valid()); // Should be true

    // Print the whole blockchain
    println!("{:#?}", blockchain);
}