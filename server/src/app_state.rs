use std::sync::{Arc, Mutex};
use blockchain::blockchain::Blockchain;

#[derive(Debug, Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl AppState {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        AppState { blockchain }
    }
}
