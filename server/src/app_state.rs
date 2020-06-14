use blockchain::blockchain::Blockchain;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl AppState {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        AppState { blockchain }
    }
}
