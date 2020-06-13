use std::sync::{Arc, Mutex};

extern crate actix_web;
extern crate blockchain;
extern crate serde_json;

mod services;
mod args;

use actix_web::{middleware, App, HttpServer};
use blockchain::blockchain::Blockchain;
use services::{blockchain_api, mine_api, transactions_api};
use args::Args;

#[derive(Debug, Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl AppState {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        AppState { blockchain }
    }
}

// watch: cargo watch -x "run -- -p 3000"
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let address = format!("0.0.0.0:{}", args.port);    

    let levicoin = Blockchain::new(address.clone());
    let levicoin = Arc::new(Mutex::new(levicoin));

    if let Some(_address) = args.master_address {
        // todo: Perform Sync
    }

    HttpServer::new(move || {
        let app_state = AppState::new(levicoin.clone());
        App::new()
            .data(app_state)
            .wrap(middleware::Logger::default())
            .service(blockchain_api::init_service())
            .service(transactions_api::init_service())
            .service(mine_api::init_service())
    })
    .bind(address)?
    .run()
    .await
}
