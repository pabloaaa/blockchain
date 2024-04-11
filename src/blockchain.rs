use crate::block::{Block};

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
        let genesis_block = Block::new(0, self.current_timestamp(), vec![], String::from("0"), 0);
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self, block: Block) {
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
    
    pub fn len(&self) -> usize {
        self.chain.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blockchain() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1); // Blockchain should be initialized with a genesis block
        assert_eq!(blockchain.chain[0].transactions.len(), 0); // Genesis block has no transactions
    }


    #[test]
    fn test_last() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.last().unwrap().transactions.len(), 0); // Genesis block has no transactions
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        let transactions = vec![
            Transaction { sender: "Alice".to_string(), receiver: "Bob".to_string(), amount: 50.0 },
            Transaction { sender: "Bob".to_string(), receiver: "Charlie".to_string(), amount: 25.0 },
        ];
        let previous_hash = blockchain.last().unwrap().hash.clone();
        let block = Block::new(1, blockchain.current_timestamp(), transactions.clone(), previous_hash, 0);
        blockchain.add_block(block);

        assert_eq!(blockchain.chain.len(), 2); // After adding a block, the blockchain length should be 2
    }
}