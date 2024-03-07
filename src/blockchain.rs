use crate::block::Block;

use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
}

#[allow(dead_code)]
impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
        };

        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, self.current_timestamp(), String::from("Genesis Block"), String::from("0"));
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self, data: String) {
        let index = self.chain.len() as u32;
        let previous_hash = self.chain[self.chain.len() - 1].hash.clone();
        let mut block = Block::new(index, self.current_timestamp(), data, previous_hash);
        block.calculate_hash();
        self.chain.push(block);
    }

    pub fn add_block_from_existing(&mut self, block: Block) {
        self.chain.push(block);
    }

    fn current_timestamp(&self) -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
    }

    pub fn last(&self) -> Option<&Block> {
        self.chain.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blockchain() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1); // Blockchain should be initialized with a genesis block
        assert_eq!(blockchain.chain[0].data, "Genesis Block");
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("New Block Data"));
        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.chain[1].data, "New Block Data");
    }

    #[test]
    fn test_last() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.last().unwrap().data, "Genesis Block");
    }

    #[test]
    fn add_block_from_existing() {
        let mut blockchain = Blockchain::new();
        let block = Block::new(1, 0, String::from("New Block Data"), String::from("0"));
        blockchain.add_block_from_existing(block);
        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.chain[1].data, "New Block Data");
    }
}