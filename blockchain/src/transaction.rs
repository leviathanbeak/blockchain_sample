use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

type Address = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    amount: u64,
    recipient: Address,
    sender: Address,
    id: u64,
}

impl Transaction {
    pub fn new(amount: u64, recipient: Address, sender: Address) -> Self {
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen();

        Transaction {
            amount,
            recipient,
            sender,
            id,
        }
    }

    pub fn stringify(&self) -> String {
        format!("{}", &self)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.amount, self.recipient, self.sender, self.id)
    }
}
