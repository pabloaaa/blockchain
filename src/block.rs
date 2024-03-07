use sha2::Sha256;
use sha2::Digest;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

#[allow(dead_code)]
impl Block {
    pub fn new(index: u32, timestamp: u64, data: String, previous_hash: String) -> Block {
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        }
    }

    pub fn calculate_hash(&mut self) {
    let mut sha = Sha256::new();
    sha.update(&format!("{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash).as_bytes());
    self.hash = format!("{:x}", sha.finalize());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hash() {
        let mut block = Block::new(0, 0, String::from("data"), String::from("previous_hash"));
        block.calculate_hash();
        assert_eq!(block.hash.len(), 64); // SHA-256 always produces a 64 character string
    }
}