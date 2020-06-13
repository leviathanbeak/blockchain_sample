#![allow(dead_code)]
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

use crate::block::{Block, Hash, NonceNumber, BlockNumber};
use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            pending_transactions: vec![],
        };

        // Genesis Block
        blockchain.create_new_block(0, "prehash".to_owned(), "myhash".to_owned());
        blockchain
    }

    // * Public Functions
    pub fn create_new_block(
        &mut self,
        nonce: NonceNumber,
        previous_block_hash: Hash,
        hash: Hash,
    ) -> Block {
        let next_index = self.get_next_index();

        let block = Block::new(
            next_index,
            nonce,
            previous_block_hash,
            hash,
            self.pending_transactions.clone(),
        );

        self.chain.push(block.clone());
        self.reset_pending_transactions();

        block
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn get_previous_hash(&self) -> String {
        self.get_last_block().unwrap().hash.clone()
    }

    pub fn create_new_transaction(&mut self, tx: Transaction) -> BlockNumber {
        self.append_tx(tx);
        self.get_last_block().unwrap().index + 1
    }

    pub fn format_pending_data(&self) -> String {
        let txs = self.pending_transactions
            .iter()
            .fold("".to_owned(), |acc, curr| acc + &curr.stringify());
        
        let index = self.get_next_index();

        format!("transactions: {}, index: {}", txs, index)
    }

    pub fn consensus(con: ConsensusOption) -> NonceNumber {
        match con {
            ConsensusOption::ProofOfWork(prev_block_hash, formated_data) => {
                let mut nonce = 0;
                let mut hash = Self::create_hash(prev_block_hash, formated_data, nonce);
                loop {
                    if &hash[..4] == "0000" {
                        break;
                    }
                    nonce += 1;
                    hash = Self::create_hash(prev_block_hash, formated_data, nonce);
                }
                nonce
            }
        }
    }

    pub fn create_hash(prev_block_hash: &str, data: &str, nonce: NonceNumber) -> String {
        let mut hasher = Sha256::new();
        let formated_data = format!("{}{}{}", prev_block_hash, data, nonce);

        hasher.update(formated_data);
        let res = hasher.finalize();

        format!("{:x}", res)
    }

    // * Private Functions
    fn append_tx(&mut self, tx: Transaction) {
        self.pending_transactions.push(tx);
    }

    fn reset_pending_transactions(&mut self) {
        self.pending_transactions = vec![];
    }

    fn get_next_index(&self) -> BlockNumber {
        match self.get_last_block() {
            Some(ref block) => block.index + 1,
            _ => 0,
        }
    }
}

pub enum ConsensusOption<'t> {
    ProofOfWork(&'t str, &'t str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block() {

        let nonce = Blockchain::consensus(ConsensusOption::ProofOfWork(
            "previousHash",
            "some stupid data",
        ));

        assert_eq!(nonce, 2901);
    }
}
