use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;

pub type BlockNumber = u64;
pub type NonceNumber = u64;
pub type Hash = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: BlockNumber,
    pub timestamp: SystemTime,
    pub transactions: Vec<Transaction>,
    pub nonce: NonceNumber,
    pub hash: Hash,
    pub previous_block_hash: Hash,
}

impl Block {
    pub fn new(
        index: BlockNumber,
        nonce: NonceNumber,
        previous_block_hash: Hash,
        hash: Hash,
        transactions: Vec<Transaction>,
    ) -> Self {
        Block {
            index,
            timestamp: SystemTime::now(),
            transactions,
            nonce,
            hash,
            previous_block_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_block() {}
}
