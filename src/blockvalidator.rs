use crate::block::Block;
use crate::blockchain::Blockchain;
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(dead_code)]
pub struct BlockValidator;

#[allow(dead_code)]
impl BlockValidator {
    pub fn new() -> BlockValidator {
        BlockValidator
    }

    pub async fn validate_and_add_block(&self, block: Block, blockchain: Arc<Mutex<Blockchain>>) -> Result<(), &'static str> {
        let mut blockchain = blockchain.lock().await;
        let last_block = blockchain.last().unwrap();

        if block.index != last_block.index + 1 {
            return Err("Block index is not valid");
        }

        if block.previous_hash != last_block.hash {
            return Err("Previous hash is not valid");
        }

        if !block.hash.starts_with("00") {
            return Err("Block hash is not valid");
        }

        blockchain.add_block(block);
        Ok(())
    }
}