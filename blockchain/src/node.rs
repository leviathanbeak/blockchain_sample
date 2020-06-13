use serde::{Deserialize, Serialize};

pub type NodeAddress = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    pub address: NodeAddress,
}

impl Node {
    pub fn new(address: NodeAddress) -> Self {
        Node { address }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
