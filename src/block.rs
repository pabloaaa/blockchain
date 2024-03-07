use sha2::Sha256;
use sha2::Digest;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

#[allow(dead_code)]
impl Block {
    pub fn new(index: u32, timestamp: u64, transactions: Vec<Transaction>, previous_hash: String, nonce: u64) -> Block {
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce,
        }
    }

    pub fn calculate_hash(&mut self) {
        let mut sha = Sha256::new();
        let transactions_string = self.transactions.iter()
            .map(|transaction| format!("{}{}{}", transaction.sender, transaction.receiver, transaction.amount))
            .collect::<Vec<String>>()
            .join("");
        sha.update(&format!("{}{}{}{}{}", self.index, self.timestamp, transactions_string, self.previous_hash, self.nonce).as_bytes());
        self.hash = format!("{:x}", sha.finalize());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hash() {
        let transactions = vec![
            Transaction { sender: String::from("Alice"), receiver: String::from("Bob"), amount: 50.0 },
            Transaction { sender: String::from("Bob"), receiver: String::from("Charlie"), amount: 25.0 },
        ];
        let mut block = Block::new(0, 0, transactions, String::from("previous_hash"), 0);
        block.calculate_hash();
        assert_eq!(block.hash.len(), 64); // SHA-256 always produces a 64 character string
    }
}