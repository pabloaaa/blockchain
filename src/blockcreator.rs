use std::{sync::Arc, time::SystemTime};

use crate::{block::{Block, Transaction}, blockvalidator::BlockValidator, blockchain::Blockchain};
use rand::Rng;
use tokio::sync::Mutex;

#[allow(dead_code)]
pub struct BlockCreator {
    validator: Arc<BlockValidator>,
}

#[allow(dead_code)]
impl BlockCreator {
    pub fn new(validator: Arc<BlockValidator>) -> BlockCreator {
        BlockCreator { validator }
    }

    pub async fn start(&self, blockchain: Arc<Mutex<Blockchain>>) {
        let mut nonce = 0;
        loop {
            let transactions = self.generate_transactions();
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let previous_hash;
            let index;
            {
                let blockchain = blockchain.lock().await;
                previous_hash = blockchain.last().unwrap().hash.clone();
                index = blockchain.len() as u32;
            }
            let mut block = Block::new(index, timestamp, transactions, previous_hash, nonce);
            while let Err(_) = self.validator.validate_and_add_block(block.clone(), Arc::clone(&blockchain)).await {
                nonce += 1;
                block.data = nonce;
                block.calculate_hash();
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    
    fn generate_transactions(&self) -> Vec<Transaction> {
        let mut rng = rand::thread_rng();
        let mut transactions = Vec::new();
        for _ in 0..rng.gen_range(1..10) {
            let sender = rng.gen_range(1..1000).to_string();
            let receiver = rng.gen_range(1..1000).to_string();
            let amount = rng.gen_range(1..1000) as f32;
            transactions.push(Transaction { sender, receiver, amount });
        }
        transactions
    }
}